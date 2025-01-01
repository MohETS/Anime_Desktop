use reqwest::Client;
use scraper::{Html, Selector};
use crate::api_jsons::{AnimeEpisodeJson, AnimeJson};
use crate::models::{Anime, AnimeEpisode};

pub struct AnimepaheApi {
    pub client: Client,
}
impl AnimepaheApi {
    pub fn new(client: Client) -> Self {
        AnimepaheApi { client }
    }


    pub async fn get_searched_anime_results(&self, anime_name: &String) -> Vec<Anime> {
        let search_response = self.client.get("https://animepahe.ru/api?m=search".to_owned() + "&q=" + &*urlencoding::encode(&anime_name) + "")
            .send().await.unwrap().text().await.unwrap();
        let search_json: AnimeJson = serde_json::from_str(&*search_response).unwrap();
        search_json.data()
    }

    pub async fn get_searched_anime_episodes(&self, anime_id: &str) -> Vec<AnimeEpisode> {
        let search_response = self.client.get("https://animepahe.ru/api?m=release".to_owned() + "&id=" + anime_id + "&sort=episode_asc&page=1")
            .send().await.unwrap().text().await.unwrap();
        let search_json: AnimeEpisodeJson = serde_json::from_str(&*search_response).unwrap();
        search_json.data()
    }

    pub async fn get_kwik_episode_url(&self,anime: &Anime, anime_episode: &AnimeEpisode) -> String {
        let search_response = self.client.get("https://animepahe.ru/play/".to_owned() + anime.anime_id() + "/" + anime_episode.session())
            .send().await.unwrap().text().await.unwrap();
        let fragment = Html::parse_fragment(&*search_response);
        let selector = Selector::parse("button").unwrap();
        let mut url_kwik = String::new();

        for element in fragment.select(&selector) {
            if element.text().collect::<Vec<_>>().get(0).unwrap().contains(&"· 1080p") {
                url_kwik = element.attr("data-src").unwrap().to_string();
                break;
            }
        }

        url_kwik

    }
}