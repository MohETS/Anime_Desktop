use reqwest::Client;

pub struct DdosGuardApi {
    pub client: Client,
}

impl DdosGuardApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_ddg2_cookie(&self) -> String {
        let ddg2_cookie_value = self.client.get("https://check.ddos-guard.net/check.js").send().await.unwrap().cookies().next().unwrap().value().to_string();
        let dd2g_cookie = "__ddg2_=".to_owned() + &*ddg2_cookie_value + "; Path=/; Domain=animepahe.ru; HttpOnly;";

        dd2g_cookie
    }
}