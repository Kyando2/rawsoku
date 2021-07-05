use serde::Deserialize;
use super::{Channel, Member, Role};
use crate::events::Cache;

#[derive(Clone, Deserialize)]
pub struct Command {
    name: Option<String>,
    id: String,
    guild_id: Option<String>,
    description: Option<String>,
    options: Option<Vec<CommandOption>>,
    default_permission: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct CommandOption {
    r#type: Option<u32>,
    name: Option<String>,
    description: Option<String>,
    required: Option<bool>,
    choices: Option<Vec<Choice>>,
    options: Option<Vec<CommandOption>>,
}

#[derive(Clone, Deserialize)]
pub struct Choice {
    name: Option<String>,
    value: Option<String>
}

impl Command {
    pub fn new_from_id(id: String) -> Command {
        let command = Command {
            id: id.as_str().into(),
            name: None,
            guild_id: None,
            description: None,
            options: None,
            default_permission: None,
        };
        command
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Command {}