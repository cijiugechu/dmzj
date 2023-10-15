use std::{fmt::Display, hash::Hash, str::FromStr, time::Duration};

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

const USER_AGENT: &str = "PostmanRuntime/7.28.4";

#[derive(Debug)]
pub struct ApiV3 {
    http_client: reqwest::Client,
}

/// condensed manga item,
/// contains only some metadata
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CondensedMangaItem {
    pub id: u32,
    pub title: String,
    pub authors: String,
    pub cover: String,
    pub last_updatetime: u64,
    pub num: u64,
    pub status: String,
    pub types: String,
}

impl PartialEq for CondensedMangaItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.last_updatetime == other.last_updatetime
    }
}

impl Hash for CondensedMangaItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.last_updatetime.hash(state);
    }
}

pub type PopularMangaItem = CondensedMangaItem;

pub type LatestUpdatesMangaItem = CondensedMangaItem;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MangaDetailsResponse {
    pub result: u16,
    pub msg: String,
    pub data: MangaDetailsData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MangaDetailsData {
    pub info: MangaDetailsInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MangaDetailsInfo {
    #[serde(
        serialize_with = "from_num_to_str",
        deserialize_with = "from_str_to_num"
    )]
    pub id: u32,
    pub title: String,
    pub subtitle: String,
    pub types: String,
    pub zone: String,
    pub status: String,
    pub last_update_chapter_name: String,
    #[serde(
        serialize_with = "from_num_to_str",
        deserialize_with = "from_str_to_num"
    )]
    pub last_updatetime: u64,
    pub cover: String,
    pub authors: String,
    pub description: String,
    pub first_letter: String,
    pub direction: String,
}

fn from_str_to_num<'de, D, Number>(deserializer: D) -> Result<Number, D::Error>
where
    D: Deserializer<'de>,
    Number: Copy + FromStr + Display,
    <Number as FromStr>::Err: Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<Number>().map_err(D::Error::custom)
}

fn from_num_to_str<S, Number>(value: &Number, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    Number: Copy + FromStr + Display + itoa::Integer,
{
    serializer.serialize_str(itoa::Buffer::new().format(*value))
}

impl ApiV3 {
    const V3_API_URL: &'static str = "https://v3api.idmzj.com";
    const API_URL: &'static str = "https://api.dmzj.com";

    pub fn popular_manga_url(page: u16) -> String {
        format!("{}/classify/0/0/{}.json", Self::V3_API_URL, page - 1)
    }

    pub fn latest_updates_url(page: u16) -> String {
        format!("{}/classify/0/1/{}.json", Self::V3_API_URL, page - 1)
    }

    pub fn manga_info_url_v1(id: u32) -> String {
        format!("{}/dynamic/comicinfo/{}.json", Self::API_URL, id)
    }

    pub fn chapter_images_url_v1(path: String) -> String {
        format!("https://m.idmzj.com/chapinfo/{}.html", path)
    }

    pub fn new() -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .build()
            .expect("failed to build http client");

        Self { http_client }
    }

    pub async fn fetch_popular_manga(
        &self,
        page: u16,
    ) -> Result<Vec<PopularMangaItem>, reqwest::Error> {
        let response = self
            .http_client
            .get(Self::popular_manga_url(page))
            .send()
            .await?;
        response.json().await
    }

    pub async fn fetch_latest_updates_manga(
        &self,
        page: u16,
    ) -> Result<Vec<LatestUpdatesMangaItem>, reqwest::Error> {
        let response = self
            .http_client
            .get(Self::latest_updates_url(page))
            .send()
            .await?;
        response.json().await
    }

    pub async fn fetch_manga_details(
        &self,
        id: u32,
    ) -> Result<MangaDetailsResponse, reqwest::Error> {
        let response = self
            .http_client
            .get(Self::manga_info_url_v1(id))
            .send()
            .await?;

        response.json().await
    }
}

impl Default for ApiV3 {
    fn default() -> Self {
        Self::new()
    }
}
