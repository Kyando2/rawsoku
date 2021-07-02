#![allow(dead_code)]
use std::collections::HashMap;

use crate::snowflakes::{Channel, Guild, Message, User};

pub type Tab<T> = HashMap<String, T>;

pub struct Cache {
    guilds: Tab<Guild>,
    channels: Tab<Channel>,
    messages: Tab<Message>,
    users: Tab<User>,
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
    pub fn new_message(&mut self, msg: Message) {
        self.messages.insert(msg.id().to_string(), msg);
    }
    pub fn new_user(&mut self, user: User) {
        self.users.insert(user.id().to_string(), user);
    }
}
