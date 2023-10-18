use dmzj_proto::comic::{ComicChapterResponse, ComicDetailResponse};
use http_cache_reqwest::{
    Cache, CacheMode, HttpCache, HttpCacheOptions, MokaManager,
};
use protobuf::Message;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use snafu::ResultExt;
use std::time::Duration;

use crate::crypto::decrypt_bytes;
use crate::error::{DmzjResult, ParseSnafu, ProtoBufSnafu, RequestSnafu};
use crate::model::{
    AuthorDetailsResponse, CategoryResponse, LatestUpdatesMangaItem,
    MangaSearchResponse, PopularMangaItem,
};

mod crypto;
/// Dmzj errors
pub mod error;
pub mod model;

/// `User-Agent` used by http client
pub const USER_AGENT: &str = "PostmanRuntime/7.28.4";

/// Dmzj API
#[derive(Debug, Clone)]
pub struct Api {
    http_client: ClientWithMiddleware,
}

impl Api {
    const V3_URL: &'static str = "https://v3api.idmzj.com";
    const V3_API_URL: &'static str = "https://nnv3api.idmzj.com";
    const V4_API_URL: &'static str = "https://nnv4api.dmzj.com";
    // const API_URL: &'static str = "https://api.dmzj.com";

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

    fn popular_manga_url(page: u16) -> String {
        format!("{}/classify/0/0/{}.json", Self::V3_URL, page)
    }

    fn latest_updates_url(page: u16) -> String {
        format!("{}/classify/0/1/{}.json", Self::V3_URL, page)
    }

    // fn manga_info_url_v1(id: u32) -> String {
    //     format!("{}/dynamic/comicinfo/{}.json", Self::API_URL, id)
    // }

    fn manga_info_url(id: u32) -> String {
        format!("{}/comic/detail/{}?uid=2665531", Self::V4_API_URL, id)
    }

    // fn chapter_images_url_v1(path: String) -> String {
    //     format!("https://m.idmzj.com/chapinfo/{}.html", path)
    // }

    fn chapter_images_url(manga_id: u32, chapter_id: i32) -> String {
        format!(
            "{}/comic/chapter/{}/{}",
            Self::V4_API_URL,
            manga_id,
            chapter_id
        )
    }

    fn category_url() -> String {
        format!("{}/0/category.json", Self::V3_API_URL)
    }

    fn author_details_url(author_tag_id: i64) -> String {
        format!("{}/UCenter/author/{}.json", Self::V3_API_URL, author_tag_id)
    }

    fn search_url<T: AsRef<str>>(keyword: T, page: u16) -> String {
        format!(
            "{}/search/show/0/{}/{}.json",
            Self::V3_API_URL,
            keyword.as_ref(),
            page
        )
    }

    pub async fn fetch_popular_manga(
        &self,
        page: u16,
    ) -> DmzjResult<Vec<PopularMangaItem>> {
        let url = Self::popular_manga_url(page);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;
        response.json().await.context(ParseSnafu)
    }

    pub async fn fetch_latest_updates_manga(
        &self,
        page: u16,
    ) -> DmzjResult<Vec<LatestUpdatesMangaItem>> {
        let url = Self::latest_updates_url(page);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;
        response.json().await.context(ParseSnafu)
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
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;

        let bytes_from_res = response.bytes().await.context(ParseSnafu)?;

        let b = decrypt_bytes(bytes_from_res)?;

        ComicDetailResponse::parse_from_bytes(&b).context(ProtoBufSnafu)
    }

    /// ```rust
    #[doc = include_str!("../examples/chapter_images.rs")]
    /// ```
    pub async fn fetch_chapter_images(
        &self,
        manga_id: u32,
        chapter_id: i32,
    ) -> DmzjResult<ComicChapterResponse> {
        let url = Self::chapter_images_url(manga_id, chapter_id);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;

        let bytes_from_res = response.bytes().await.context(ParseSnafu)?;

        let b = decrypt_bytes(bytes_from_res)?;

        ComicChapterResponse::parse_from_bytes(&b).context(ProtoBufSnafu)
    }

    /// ```rust
    #[doc = include_str!("../examples/category.rs")]
    /// ```
    pub async fn fetch_category(&self) -> DmzjResult<CategoryResponse> {
        let url = Self::category_url();

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;

        response.json().await.context(ParseSnafu)
    }

    /// ```rust
    #[doc = include_str!("../examples/author_details.rs")]
    /// ```
    pub async fn fetch_author_details(
        &self,
        author_tag_id: i64,
    ) -> DmzjResult<AuthorDetailsResponse> {
        let url = Self::author_details_url(author_tag_id);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;

        response.json().await.context(ParseSnafu)
    }

    /// ```rust
    #[doc = include_str!("../examples/search.rs")]
    /// ```
    pub async fn search_manga<T: AsRef<str>>(
        &self,
        keyword: T,
        page: u16,
    ) -> DmzjResult<MangaSearchResponse> {
        let url = Self::search_url(keyword, page);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .context(RequestSnafu)?;

        response.json().await.context(ParseSnafu)
    }
}

impl Default for Api {
    fn default() -> Self {
        Self::new()
    }
}
