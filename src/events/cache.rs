#![allow(dead_code)]
use std::collections::HashMap;

use crate::snowflakes::Guild;

pub type Tab<T> = HashMap<String, T>;

pub struct Cache {
    guilds: Tab<Guild>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            guilds: HashMap::new(),
        }
    }
    pub fn new_guild(&mut self, guild: Guild) {
        self.guilds.insert(guild.id().to_string(), guild);
    }
}
