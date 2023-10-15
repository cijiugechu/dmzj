use crate::crypto::{decrypt, get_private_key};
use dmzj_proto::details::ComicDetailResponse;
use protobuf::Message;

use std::time::Duration;

mod crypto;

#[derive(Debug)]
pub struct ApiV4 {
    http_client: reqwest::Client,
}

impl ApiV4 {
    const V4_API_URL: &'static str = "https://nnv4api.dmzj.com";

    pub fn new() -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build http client");

        Self { http_client }
    }

    pub fn manga_info_url(id: u32) -> String {
        format!("{}/comic/detail/{}?uid=2665531", Self::V4_API_URL, id)
    }

    /// ```rust
    #[doc = include_str!("../examples/parse.rs")]
    /// ```
    pub async fn fetch_manga_details(
        &self,
        id: u32,
    ) -> Result<ComicDetailResponse, reqwest::Error> {
        let response = self
            .http_client
            .get(Self::manga_info_url(id))
            .send()
            .await?;

        let text = response.text().await?;

        let cipher = get_private_key();

        let b = decrypt(text, cipher);

        Ok(ComicDetailResponse::parse_from_bytes(&b).unwrap())
    }
}

impl Default for ApiV4 {
    fn default() -> Self {
        Self::new()
    }
}
