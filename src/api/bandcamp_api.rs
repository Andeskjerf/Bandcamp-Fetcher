use reqwest::RequestBuilder;

enum RequestType {
    Get,
    Post,
}

pub struct BandcampAPI {
    client: reqwest::Client,
    identity: String,
}

impl BandcampAPI {
    pub fn new(identity: &str) -> Self {
        let builder = reqwest::ClientBuilder::new().cookie_store(true);
        Self {
            client: builder.build().expect("failed to build client"),
            identity: identity.to_string(),
        }
    }

    fn make_request(self, request_type: RequestType, url: &str) -> RequestBuilder {
        let request_builder = match request_type {
            RequestType::Get => self.client.get(url),
            RequestType::Post => self.client.post(url),
        };

        request_builder.header("Cookie", format!("identity={}", self.identity))
    }

    // async fn get_collection_summary(self) -> Result<reqwest::Response, reqwest::Error> {
    pub async fn get_collection_summary(self) {
        let request = self.make_request(
            RequestType::Get,
            "https://bandcamp.com/api/fan/2/collection_summary",
        );
    }
}
