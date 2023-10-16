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
