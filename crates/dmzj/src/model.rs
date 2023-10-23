use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use std::{fmt::Display, hash::Hash, str::FromStr};

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

fn from_num_to_str<S, Number>(
    value: &Number,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    Number: Copy + FromStr + Display + itoa::Integer,
{
    serializer.serialize_str(itoa::Buffer::new().format(*value))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryResponse {
    pub code: u16,
    pub msg: String,
    pub data: Vec<CategoryItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryItem {
    pub tag_id: u32,
    pub title: String,
    pub cover: String,
}

impl PartialEq for CategoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.tag_id == other.tag_id
    }
}

impl Hash for CategoryItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag_id.hash(state);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthorDetailsResponse {
    pub nickname: String,
    pub data: Vec<MangaByAuthor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaByAuthor {
    pub id: u32,
    pub name: String,
    pub cover: String,
    pub status: String,
}

impl PartialEq for MangaByAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.status == other.status
    }
}

impl Hash for MangaByAuthor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.status.hash(state);
    }
}

pub type MangaSearchResponse = Vec<SearchedMangaItem>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedMangaItem {
    pub id: u32,
    pub addtime: i64,
    pub alias_name: String,
    pub authors: String,
    pub copyright: i32,
    pub cover: String,
    pub device_show: i32,
    pub grade: i32,
    pub hidden: i32,
    pub hot_hits: u32,
    pub is_safe: i32,
    pub last_name: String,
    pub quality: i32,
    pub title: String,
    pub types: String,
}
