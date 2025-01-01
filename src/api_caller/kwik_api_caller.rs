use radix_fmt::radix;
use reqwest::header::REFERER;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct KwikApi {
    client: Client,
}
impl KwikApi {
    pub fn new(client: Client) -> Self {
        KwikApi {
            client
        }
    }

    pub async fn get_m3u8_url(&mut self, kwik_url: &str) -> String {
        let search_response = self.client.get(kwik_url)
            .header(REFERER, "https://animepahe.ru/")
            .send().await.unwrap().text().await.unwrap();

        let mut m3u8_url_info = String::new();
        let fragment = Html::parse_fragment(&*search_response);
        let selector = Selector::parse("script").unwrap();

        for element in fragment.select(&selector) {
            if !element.text().collect::<Vec<_>>().is_empty() {
                m3u8_url_info = element.text().collect::<Vec<_>>().get(0).unwrap().to_string();
                break;
            }
        }

        m3u8_url_info = m3u8_url_info[1783..m3u8_url_info.len()].to_string();

        let mut temp = m3u8_url_info.split(";',").collect::<Vec<&str>>();

        let mut p: &str = &temp[0][8..];
        if p.contains("1J/H") {
            p = &p[0..32];
        } else {
            p = p[0..34].trim_matches(';').trim_matches('\'').trim_matches('\\');
        }

        temp = temp[1].split(",").collect::<Vec<&str>>();
        let a: usize = temp[0].parse::<usize>().unwrap();
        let c: usize = temp[1].parse::<usize>().unwrap();
        let k: Vec<&str> = temp[2][1..(temp[2].len() - 12)].split("|").collect();
        let link = Self::animepahe_link_parser(p, c, a, k);

        link
    }

    /** Method from Animepahe.ru to get the episode's .m3u8 link converted in Rust **/
    pub fn animepahe_link_parser(p: &str, mut c: usize, a: usize, k: Vec<&str>) -> String {
        let mut link = p.to_string();

        while c > 0 {
            c -= 1;
            if !k[c].is_empty() {
                let regex = regex::Regex::new(&format!(r"\b{}\b", Self::e(c, a)));
                link = regex.unwrap().replace_all(&link, k[c]).to_string();
            }
        }

        link
    }

    fn e(c: usize, a: usize) -> String {
        let mut result: String = String::new();

        if c < a {} else {
            result = Self::e(c / a, a);
        }

        let c = c % a;
        if c > 35 {
            result = result + &*std::char::from_u32((c + 29) as u32).unwrap().to_string();
        } else {
            result = result + &*radix(c, 36).to_string();
        }

        result
    }
}