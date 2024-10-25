use reqwest::header::REFERER;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct KwikApi {
    client: Client,
    m3u8_url_info_parts: Vec<String>,
}
impl KwikApi {
    pub fn new(client: Client) -> Self {
        KwikApi {
            client
            ,
            m3u8_url_info_parts: Vec::new(),
        }
    }

    pub async fn get_m3u8_url(&mut self, kwik_url: &str) -> String {
        let search_response = self.client.get(kwik_url)
            .header(REFERER, "https://animepahe.ru/")
            .send().await.unwrap().text().await.unwrap();
        self.get_m3u8_url_info(search_response);
        println!("{:?}", self.m3u8_url_info_parts);
        "https:".to_owned() + &*self.m3u8_url_info_parts[0] + "/stream/" + &*self.m3u8_url_info_parts[1] + "/"
            + &*self.m3u8_url_info_parts[2] + "/" + &*self.m3u8_url_info_parts[3] + "/uwu.m3u8"
    }

    fn get_m3u8_url_info(&mut self, search_response: String) {
        let fragment = Html::parse_fragment(&*search_response);
        let mut selector = Selector::parse("link").unwrap();
        for element in fragment.select(&selector) {
            if element.attr("rel").unwrap().eq("preconnect") && element.attr("href").unwrap().contains("nextcdn.org") {
                self.m3u8_url_info_parts.push(element.attr("href").unwrap().to_string());
                self.m3u8_url_info_parts.push(element.attr("href").unwrap()[5..7].to_string());
                break;
            }
        }

        selector = Selector::parse("script").unwrap();
        let mut m3u8_url_info = String::new();
        for element in fragment.select(&selector) {
            if !element.text().collect::<Vec<_>>().is_empty() {
                m3u8_url_info = element.text().collect::<Vec<_>>().get(0).unwrap().to_string();
                break;
            }
        }

        let value_position_checker = "/1D/1C/1B.1A";
        m3u8_url_info = m3u8_url_info[1783..m3u8_url_info.len()].to_string();
        let m3u8_url_info_length: usize = m3u8_url_info.len();

        /** Checks if the kwik contains the String value_position_checker
                        * If it's true the value is at the end with the episode's code
                        * If not that means the value is at the start
        */
        if m3u8_url_info.contains(value_position_checker) {
            let string = &m3u8_url_info[(m3u8_url_info_length - 121)..(m3u8_url_info_length - 54)];

            let temp = &mut string.split("|").collect::<Vec<&str>>();
            self.m3u8_url_info_parts.push(temp[1].to_string());
            self.m3u8_url_info_parts.push(temp[0].to_string());
        } else {
            self.m3u8_url_info_parts.push(m3u8_url_info[34..36].to_string());
            self.m3u8_url_info_parts.push(m3u8_url_info[(m3u8_url_info_length - 118)..(m3u8_url_info_length - 54)].to_string());
        }
    }
}