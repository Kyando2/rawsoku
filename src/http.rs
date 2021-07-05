use reqwest::blocking::Client;

use crate::snowflakes::User;

pub struct Handle {
    client: reqwest::blocking::Client,
    me: User,
    auth: String,
}

impl Handle {
    pub fn new(me: User, auth: String) -> Handle {
        Handle {
            client: Client::new(),
            me,
            auth,
        }
    }
    pub fn post(&self, url: String, body: String) -> reqwest::Result<reqwest::blocking::Response> {
        self.client
            .post(url)
            .header("Authorization", format!("Bot {}", self.auth.clone()))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
    }
    pub fn me(&self) -> &User {
        &self.me
    }
}
