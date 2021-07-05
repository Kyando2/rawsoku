use std::str;

use serde::Deserialize;
use serde_json::json;

use crate::{consts::BASE_URL, prelude::Handle};

use super::{Channel, User};

#[derive(Clone, Deserialize)]
pub struct Message {
    id: String,
    channel_id: String,
    guild_id: String,
    author: User,
    content: String,
}

impl Message {
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    pub fn channel(&self) -> Channel {
        Channel::new_from_id(self.channel_id.clone())
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn author(&self) -> &User {
        &self.author
    }
    pub fn reply(&self, handle: Handle, content: String) {
        let url = format!("{}/channels/{}/messages", BASE_URL, self.channel_id.to_string());
        let _ = handle
            .post(
                url,
                json!({
                    "content": content,
                    "tts": false,
                    "message_reference": {
                        "message_id": self.id,
                        "channel_id": self.channel_id,
                        "guild_id": self.guild_id,
                        "fail_if_not_exists": false,
                    } 
                })
                .to_string(),
            )
            .expect("Error sending request");
    }
}
