use serde_json::Value;

use crate::events::Cache;

#[derive(Clone)]
pub struct Channel {
    id: String,
    name: String,
}

impl Channel {
    pub fn gen_create_guild(data: &Value, cache: &mut Cache) -> Channel {
        let name = data["name"].as_str().unwrap().to_owned();
        let id = data["id"].as_str().unwrap().to_owned();
        let channel = Channel {
            id,
            name
        };
        cache.new_channel(channel.clone());
        channel
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}