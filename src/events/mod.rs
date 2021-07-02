mod cache;

pub use cache::Cache;
use reqwest::blocking::Client;
use serde_json::Value;  

use crate::{consts::{dispatch::*, op_code::*}, snowflakes::Guild};
use crate::snowflakes::Channel;

pub enum Flash {
    HeartbeatAck,
    GuildCreate,
}

struct Handlers {
    on_message: Option<fn(Client, Channel)>,
}


pub struct Wax {
    data: Cache,
    handlers: Handlers,
    auth_token: String,
}

impl Wax {
    pub fn new(auth_token: String) -> Wax {
        Wax {
            data: Cache::new(),
            handlers: Handlers {
                on_message: None,
            },
            auth_token
        }
    }
    pub fn handle(&mut self, data: Value) {
        let op_code = data["op"]
            .as_u64()
            .expect("The gateway sent a message containing an invalid op code") as u8;
        if op_code == HEARTBEAT_ACK {
            println!("A heartbeat was acknowledged!");
        } else if op_code == DISPATCH {
            println!(
                "Message delivery {}", 
                data["t"]
                .as_str()
                .expect("The gateway sent a dispatch containing an invalid event name")
            );
            if data["t"] == GUILD_CREATE {
                self.handle_guild_create(data);
            } else if data["t"] == MESSAGE_CREATE {
                self.handle_message(data);
            }
        }
    }
    pub fn set_on_message(&mut self, handler: Option<fn(Client, Channel)>) {
        self.handlers.on_message = handler;
    }
    fn handle_guild_create(&mut self, data: Value) {
        let guild_data = &data["d"];
        let _ = Guild::gen_create_guild(guild_data, &mut self.data); // Caches the guild
    }
    fn handle_message(&mut self, data: Value) {
        let channel = Channel::gen_create_message(&data["d"], &mut self.data, self.auth_token.clone());
        if self.handlers.on_message.is_some() {
            let me = self.handlers.on_message.unwrap();
            std::thread::spawn(move || {
                me(Client::new(), channel);
            });
        }
    }
}