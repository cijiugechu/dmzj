use dmzj_proto::details::ComicDetailResponse;
use http_cache_reqwest::{
    Cache, CacheMode, HttpCache, HttpCacheOptions, MokaManager,
};
use protobuf::Message;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use snafu::ResultExt;
use std::time::Duration;

use crate::crypto::{decrypt, get_private_key};
use crate::error::{DmzjResult, ParseSnafu, RequestSnafu};
use crate::model::{LatestUpdatesMangaItem, PopularMangaItem};

mod crypto;
pub mod error;
pub mod model;

/// `User-Agent` used by http client
pub const USER_AGENT: &str = "PostmanRuntime/7.28.4";

#[derive(Debug, Clone)]
pub struct Api {
    http_client: ClientWithMiddleware,
}

impl Api {
    const V3_API_URL: &'static str = "https://v3api.idmzj.com";
    const V4_API_URL: &'static str = "https://nnv4api.dmzj.com";
    const API_URL: &'static str = "https://api.dmzj.com";

    pub fn new() -> Self {
        let http_client = ClientBuilder::new(
            reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to build http client"),
        )
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: MokaManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();

        Self { http_client }
    }

    pub fn popular_manga_url(page: u16) -> String {
        format!("{}/classify/0/0/{}.json", Self::V3_API_URL, page - 1)
    }

    pub fn latest_updates_url(page: u16) -> String {
        format!("{}/classify/0/1/{}.json", Self::V3_API_URL, page - 1)
    }

    pub fn manga_info_url_v1(id: u32) -> String {
        format!("{}/dynamic/comicinfo/{}.json", Self::API_URL, id)
    }

    pub fn manga_info_url(id: u32) -> String {
        format!("{}/comic/detail/{}?uid=2665531", Self::V4_API_URL, id)
    }

    pub fn chapter_images_url_v1(path: String) -> String {
        format!("https://m.idmzj.com/chapinfo/{}.html", path)
    }

    pub async fn fetch_popular_manga(
        &self,
        page: u16,
    ) -> DmzjResult<Vec<PopularMangaItem>> {
        let url = Self::popular_manga_url(page);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context(RequestSnafu { url: url.as_str() })?;
        response
            .json()
            .await
            .context(ParseSnafu { url: url.as_str() })
    }

    pub async fn fetch_latest_updates_manga(
        &self,
        page: u16,
    ) -> DmzjResult<Vec<LatestUpdatesMangaItem>> {
        let url = Self::latest_updates_url(page);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context(RequestSnafu { url: url.as_str() })?;
        response
            .json()
            .await
            .context(ParseSnafu { url: url.as_str() })
    }

    /// ```rust
    #[doc = include_str!("../examples/parse.rs")]
    /// ```
    pub async fn fetch_manga_details(
        &self,
        id: u32,
    ) -> DmzjResult<ComicDetailResponse> {
        let url = Self::manga_info_url(id);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context(RequestSnafu { url: url.as_str() })?;

        let text = response
            .text()
            .await
            .context(ParseSnafu { url: url.as_str() })?;

        let cipher = get_private_key();

        let b = decrypt(text, cipher)?;

        Ok(ComicDetailResponse::parse_from_bytes(&b).unwrap())
    }
}

impl Default for Api {
    fn default() -> Self {
        Self::new()
    }
}
