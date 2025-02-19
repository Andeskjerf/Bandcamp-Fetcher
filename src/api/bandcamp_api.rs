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

    fn make_request(&self, request_type: RequestType, url: &str) -> RequestBuilder {
        let request_builder = match request_type {
            RequestType::Get => self.client.get(url),
            RequestType::Post => self.client.post(url),
        };

        request_builder.header("Cookie", format!("identity={}", self.identity))
    }

    pub fn get_collection_summary(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.make_request(
            RequestType::Get,
            "https://bandcamp.com/api/fan/2/collection_summary",
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
        )
        .send()
    }

    pub fn get_download_page_html(
        &self,
        url: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.make_request(RequestType::Get, &format!("{url}"))
            .send()
    }
}
