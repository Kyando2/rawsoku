use serde::Deserialize;
use serde_json::json;

use crate::consts::BASE_URL;
use crate::prelude::Handle;

use super::User;

#[derive(Clone, Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    r#type: Option<u32>,
    guild_id: Option<String>,
    position: Option<u32>,
    icon: Option<String>,
    recipients: Option<Vec<User>>,
    nsfw: Option<bool>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u32>,
    topic: Option<String>,
}

impl Channel {
    pub fn new_from_id(id: String) -> Channel {
        Channel {
            id: id.into(),
            name: "Null".to_string(),
            r#type: None,
            guild_id: None,
            position: None,
            topic: None,
            recipients: None,
            nsfw: None,
            bitrate: None,
            user_limit: None,
            icon: None,
            rate_limit_per_user: None,
        }
    }
    pub fn send_message(&self, handle: Handle, content: String) {
        let url = format!("{}/channels/{}/messages", BASE_URL, self.id.to_string());
        let _ = handle
            .post(
                url,
                json!({
                    "content": content,
                    "tts": false,
                })
                .to_string(),
            )
            .expect("Error sending request");
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}
