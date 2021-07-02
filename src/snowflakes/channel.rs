use serde_json::Value;
use serde_json::json;

use crate::events::Cache;
use reqwest::blocking::Client;
use crate::consts::BASE_URL;

#[derive(Clone)]
pub struct Channel {
    id: String,
    name: Option<String>,
    auth: Option<String>,
}

impl Channel {
    pub fn gen_create_guild(data: &Value, cache: &mut Cache) -> Channel {
        let name = data["name"].as_str().unwrap().to_owned();
        let id = data["id"].as_str().unwrap().to_owned();
        let channel = Channel {
            id,
            name: Some(name),
            auth: None
        };
        cache.new_channel(channel.clone());
        channel
    }
    pub fn gen_create_message(data: &Value, cache: &mut Cache, auth: String) -> Channel {
        let id = data["channel_id"].as_str().unwrap().to_owned();
        let channel = Channel {
            id,
            name: None,
            auth: Some(auth)
        };
        cache.new_channel(channel.clone());
        channel
    }
    pub fn send_message(&self, client: Client, content: String) {
        let url = format!("{}/channels/{}/messages", BASE_URL, self.id);
        let dat = client.post(url)
            .header("Authorization", format!("Bot {}", self.auth.as_ref().unwrap().clone()))
            .header("Content-Type", "application/json")
            .body(json!({
                "content": content,
                "tts": false,
            }).to_string())
            .send().expect("guess not");
        println!("{}", dat.text().unwrap());
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}