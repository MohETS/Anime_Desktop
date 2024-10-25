use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Anime {
    #[serde(alias = "session")]
    anime_id: String,
    #[serde(alias = "type")]
    anime_type: String,
    #[serde(alias = "poster")]
    cover: String,
    id: u16,
    title: String,
    episodes: u16,
    status: String,
    season: String,
    year: i16,
    score: Option<f32>,
}

impl Anime {
    pub fn anime_id(&self) -> &str {
        &self.anime_id
    }

    pub fn anime_type(&self) -> &str {
        &self.anime_type
    }

    pub fn cover(&self) -> &str {
        &self.cover
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn episodes(&self) -> u16 {
        self.episodes
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn season(&self) -> &str {
        &self.season
    }

    pub fn year(&self) -> i16 {
        self.year
    }

    pub fn score(&self) -> Option<f32> {
        self.score
    }
}

