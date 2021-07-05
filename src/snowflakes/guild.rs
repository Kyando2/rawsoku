use serde::Deserialize;
use super::{Channel, Member, Role};
use crate::events::Cache;

#[derive(Clone, Deserialize)]
pub struct Guild {
    name: Option<String>,
    id: String,
    channels: Option<Vec<Channel>>,
    members: Option<Vec<Member>>,
    roles: Option<Vec<Role>>,
}

impl Guild {
    pub fn new_from_id(id: String, cache: &mut Cache) -> Guild {
        let guild = Guild {
            id: id.as_str().into(),
            name: None,
            channels: None,
            members: None,
            roles: None,
        };
        cache.new_guild(guild.clone());
        guild
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
}
