use std::time::Duration;

use log::info;
// use reqwest::RequestBuilder;
use reqwest::blocking::RequestBuilder;

enum RequestType {
    Get,
    Post,
}

pub struct BandcampAPI {
    client: reqwest::blocking::Client,
    username: String,
    identity: String,
}

impl BandcampAPI {
    pub fn new(username: &str, identity: &str) -> Self {
        let builder = reqwest::blocking::ClientBuilder::new().cookie_store(true);
        Self {
            client: builder.build().expect("failed to build client"),
            username: username.to_string(),
            identity: identity.to_string(),
        }
    }

    fn make_request(
        &self,
        request_type: RequestType,
        url: &str,
        timeout: Option<u64>,
    ) -> RequestBuilder {
        let request_builder = match request_type {
            RequestType::Get => self.client.get(url),
            RequestType::Post => self.client.post(url),
        };

        request_builder
            .header("Cookie", format!("identity={}", self.identity))
            .timeout(Duration::from_secs(timeout.unwrap_or(30)))
    }

    pub fn get_collection_items_json(
        &self,
        fan_id: u64,
        count: u32,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        // use the oldest possible token to get all results
        let token = "9999999999:9999999999:t::";
        self.make_request(
            RequestType::Post,
            "https://bandcamp.com/api/fancollection/1/collection_items",
            None,
        )
        .body(format!(
            "{{\"fan_id\":{fan_id},\"older_than_token\":\"{token}\",\"count\":{count}}}"
        ))
        .header("Accept", "application/json, text/javascript, */*; q=0.01")
        .send()
    }

    pub fn get_collection_summary(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.make_request(
            RequestType::Get,
            "https://bandcamp.com/api/fan/2/collection_summary",
            None,
        )
        .header("Accept", "application/json, text/javascript, */*; q=0.01")
        .send()
    }

    pub fn get_collection_summary_html(
        &self,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.make_request(
            RequestType::Get,
            &format!("https://bandcamp.com/{}", self.username),
            None,
        )
        .send()
    }

    pub fn get_download_page_html(
        &self,
        url: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.make_request(RequestType::Get, url, None).send()
    }

    // does it make sense to have this function here?
    pub fn download_file(&self, url: &str, path: &str) -> String {
        let mut response = self
            // at worst, an album is surely not larger than a GB?
            // is 30 minutes timeout too much?
            .make_request(RequestType::Get, url, Some(60 * 30))
            .send()
            .expect("unable to make request!");

        let content = String::from_utf8(
            response
                .headers()
                .get("content-disposition")
                .expect("unable to get content_disposition header")
                .as_bytes()
                .to_vec(),
        )
        .expect("unable to convert header value to string!");

        let split = content.split(';').collect::<Vec<&str>>();
        // we know the position of the different elements, so lets get them naively
        // not very robust
        let filename = split[1].split(r#"""#).collect::<Vec<&str>>()[1];

        let path = format!("{}/{}", path, filename);
        let mut file = std::fs::File::create(&path).expect("unable to create file for download!");

        // copy request bytes into file
        std::io::copy(&mut response, &mut file).expect("failed to copy response to file");

        path
    }
}
