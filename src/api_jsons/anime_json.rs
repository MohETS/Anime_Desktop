use serde::{Deserialize, Serialize};
use crate::models::Anime;

#[derive(Serialize, Deserialize,Default, Debug)]
#[serde(default)]
pub struct AnimeJson {
    data: Vec<Anime>
}

impl AnimeJson {
    pub fn data(self) -> Vec<Anime> {
        self.data
    }
}