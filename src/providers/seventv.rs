use serde::{Deserialize, Serialize};

use super::{Provider, ProviderEmotes};

impl From<SevenTvSet> for ProviderEmotes {
    fn from(set: SevenTvSet) -> Self {
        let emotes = set
            .emotes
            .into_iter()
            .map(|emote| {
                let file = &emote
                    .data
                    .host
                    .files
                    .iter()
                    .find(|file| file.name == Name::The4XAvif)
                    .unwrap();

                let name = emote.id;
                let url = format!("https:{}/{}", emote.data.host.url, file.name);

                trace!("Found emote: {} -> {}", name, url);

                super::Emote {
                    name,
                    url,
                    size: super::Size::X4,
                    extension: super::FileType::Avif,
                }
            })
            .collect();

        ProviderEmotes {
            provider: super::Providers::SevenTv,
            emotes,
        }
    }
}

impl Provider for SevenTvSet {
    const BASE_URL: &'static str = "https://7tv.io/v3/emote-sets/";

    #[instrument(level = tracing::Level::TRACE)]
    fn get(id: &str) -> Result<Self, super::ProviderError> {
        let url = format!("{}{}", Self::BASE_URL, id);
        trace!("Fetching emotes from {}", url);

        let resp = reqwest::blocking::get(&url)?.text()?;

        debug!("Got reponse {}", resp);

        let resp_json = serde_json::from_str(&resp)?;

        Ok(resp_json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SevenTvSet {
    pub id: String,
    pub name: String,
    pub tags: Vec<Option<serde_json::Value>>,
    pub immutable: bool,
    pub privileged: bool,
    pub emotes: Vec<Emote>,
    pub capacity: i64,
    pub owner: Owner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub flags: i64,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,
    pub actor_id: Option<String>,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub flags: i64,
    pub tags: Vec<Option<serde_json::Value>>,
    pub lifecycle: i64,
    pub listed: bool,
    pub animated: bool,
    pub owner: Owner,
    pub host: Host,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub url: String,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: Name,
    pub width: i64,
    pub height: i64,
    pub frame_count: Option<i64>,
    pub size: Option<i64>,
    pub format: Format,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "AVIF")]
    Avif,
    #[serde(rename = "WEBP")]
    Webp,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Name {
    #[serde(rename = "1x")]
    The1X,
    #[serde(rename = "1x.avif")]
    The1XAvif,
    #[serde(rename = "2x")]
    The2X,
    #[serde(rename = "2x.avif")]
    The2XAvif,
    #[serde(rename = "3x")]
    The3X,
    #[serde(rename = "3x.avif")]
    The3XAvif,
    #[serde(rename = "4x")]
    The4X,
    #[serde(rename = "4x.avif")]
    The4XAvif,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = serde_json::to_string(self).unwrap().replace('"', "");
        write!(f, "{}", name)
    }
}
