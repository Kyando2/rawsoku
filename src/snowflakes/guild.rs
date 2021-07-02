use serde_json::Value;

use super::{Channel, Role, User};

use crate::events::Cache;

#[derive(Clone)]
pub struct Guild {
    name: String,
    id: String,
    auth: String,
    channels: Option<Vec<Channel>>,
    members: Option<Vec<User>>,
    roles: Option<Vec<Role>>,
}

impl Guild {
    pub fn new_from_object(data: &Value, cache: &mut Cache, auth: String) -> Guild {
        let name = data["name"]
            .as_str()
            .expect("The guild's name wasn't a string")
            .to_owned();
        let id = data["id"].as_str().unwrap().to_owned();
        let maybe_channels = data.get("channels");
        let channels = match maybe_channels {
            Some(channels_data) => Some(
                channels_data
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|x| Channel::new_from_object(x, cache, auth.clone()))
                    .collect(),
            ),
            None => None,
        };
        let maybe_members = data.get("members");
        let members = match maybe_members {
            Some(members_data) => Some(
                members_data
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|x| User::new_from_object(&x["user"], cache, auth.clone()))
                    .collect(),
            ),
            None => None,
        };
        let maybe_roles = data.get("roles");
        let roles = match maybe_roles {
            Some(roles_data) => Some(
                roles_data
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|x| Role::new_from_object(x))
                    .collect(),
            ),
            None => None,
        };
        let guild = Guild {
            name,
            id,
            channels,
            members,
            roles,
            auth,
        };
        cache.new_guild(guild.clone());
        guild
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}
