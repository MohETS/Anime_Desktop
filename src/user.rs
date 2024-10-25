use std::sync::Arc;
use reqwest::{Client, ClientBuilder, Url};
use reqwest::cookie::Jar;

pub struct User {
    client: Client,
    cookie_store: Arc<Jar>,
}

impl User {
    pub fn new() -> Self {
        let cookie_store = Arc::new(Jar::default());
        let client_builder = ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36".to_string())
            .cookie_provider(cookie_store.clone());
        let client = client_builder.build().unwrap();

        User {
            client,
            cookie_store,
        }
    }

    pub fn add_cookie(&self, cookie: &str, url: &Url) {
        self.cookie_store.add_cookie_str(cookie, url);
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}