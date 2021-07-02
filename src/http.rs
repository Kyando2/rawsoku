use reqwest::blocking::Client;

use crate::snowflakes::User;

pub struct Handle {
    client: reqwest::blocking::Client,
    me: User,
}

impl Handle {
    pub fn new(me: User) -> Handle {
        Handle {
            client: Client::new(),
            me
        }
    }
    pub fn post(
        &self,
        auth: String,
        url: String,
        body: String,
    ) -> reqwest::Result<reqwest::blocking::Response> {
        self.client
            .post(url)
            .header("Authorization", format!("Bot {}", auth))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
    }
    pub fn me(&self) -> &User {
        &self.me
    }
}
