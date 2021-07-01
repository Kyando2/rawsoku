use std::collections::HashMap;

use crate::snowflakes::{Channel, Guild, Message, User};

pub type tab<T> = HashMap<String, T>;

pub struct Cache {
    guilds: tab<Guild>,
    channels: tab<Channel>,
    messages: tab<Message>,
    users: tab<User>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            guilds: HashMap::new(),
            channels: HashMap::new(),
            messages: HashMap::new(),
            users: HashMap::new(),
        }
    }
    pub fn new_guild(&mut self, guild: Guild) {
        self.guilds.insert(guild.id().to_string(), guild);
    }
    pub fn new_channel(&mut self, channel: Channel) {
        self.channels.insert(channel.id().to_string(), channel);
    }
}