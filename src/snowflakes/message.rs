use std::str;

use serde_json::Value;

use crate::events::Cache;

use super::{Channel, User};

#[derive(Clone)]
pub struct Message {
    id: String,
    auth: String,
    channel: Channel,
    author: User,
    content: String,
}

impl Message {
    pub fn new_from_object(data: &Value, cache: &mut Cache, auth: String) -> Message {
        let channel = Channel::new_from_id(
            data["channel_id"].as_str().unwrap().to_owned(),
            cache,
            auth.clone(),
        );
        let author = User::new_from_object(&data["author"], cache, auth.clone());
        let id = data["id"].as_str().unwrap().to_owned();
        let msg = Message {
            id,
            auth,
            channel,
            author,
            content: data["content"].as_str().unwrap().to_owned(),
        };
        cache.new_message(msg.clone());
        msg
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn author(&self) -> &User {
        &self.author
    }
}
