use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Default, Debug)]
#[serde(default)]
pub struct AnimeEpisode {
    id: u16,
    anime_id: u16,
    episode: u16,
    edition: String,
    title: String,
    #[serde(alias = "snapshot")]
    thumbnail: String,
    disc:String,
    audio:String,
    duration:String,
    session: String,
    filler: i8,
    created_at: String,
}

impl AnimeEpisode {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn anime_id(&self) -> u16 {
        self.anime_id
    }

    pub fn episode(&self) -> u16 {
        self.episode
    }

    pub fn edition(&self) -> &str {
        &self.edition
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn thumbnail(&self) -> &str {
        &self.thumbnail
    }

    pub fn disc(&self) -> &str {
        &self.disc
    }

    pub fn audio(&self) -> &str {
        &self.audio
    }

    pub fn duration(&self) -> &str {
        &self.duration
    }

    pub fn session(&self) -> &str {
        &self.session
    }

    pub fn filler(&self) -> i8 {
        self.filler
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}