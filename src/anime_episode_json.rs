use serde::{Deserialize, Serialize};
use crate::anime_episode::AnimeEpisode;

#[derive(Serialize, Deserialize,Default, Debug)]
#[serde(default)]
pub struct AnimeEpisodeJson {
    data: Vec<AnimeEpisode>
}

impl AnimeEpisodeJson {
    pub fn data(&self) -> &Vec<AnimeEpisode> {
        &self.data
    }
}