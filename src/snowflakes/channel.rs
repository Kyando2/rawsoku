use serde_json::json;
use serde_json::Value;

use crate::consts::BASE_URL;
use crate::events::Cache;
use crate::prelude::Handle;

#[derive(Clone)]
pub struct Channel {
    id: String,
    auth: String,
    name: Option<String>,
}

impl Channel {
    pub fn new_from_object(data: &Value, cache: &mut Cache, auth: String) -> Channel {
        let name = data["name"].as_str().unwrap().to_owned();
        let id = data["id"].as_str().unwrap().to_owned();
        let channel = Channel {
            id,
            name: Some(name),
            auth,
        };
        cache.new_channel(channel.clone());
        channel
    }
    pub fn new_from_id(id: String, cache: &mut Cache, auth: String) -> Channel {
        let channel = Channel {
            id,
            name: None,
            auth,
        };
        cache.new_channel(channel.clone());
        channel
    }
    pub fn send_message(&self, handle: Handle, content: String) {
        let url = format!("{}/channels/{}/messages", BASE_URL, self.id);
        handle
            .post(
                self.auth.clone(),
                url,
                json!({
                    "content": content,
                    "tts": false,
                })
                .to_string(),
            )
            .expect("Error sending request");
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}
