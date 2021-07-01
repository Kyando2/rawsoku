mod cache;

pub use cache::Cache;
use reqwest::blocking::Client;
use serde_json::Value;  

use crate::{consts::{dispatch::*, op_code::*}, snowflakes::Guild};

pub enum Flash {
    HeartbeatAck,
    GuildCreate,
}

pub struct Wax {
    data: Cache,
}

impl Wax {
    pub fn new() -> Wax {
        Wax {
            data: Cache::new(),
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
            }
        }
    }
    fn handle_guild_create(&mut self, data: Value) {
        let guild_data = &data["d"];
        let _ = Guild::gen_create_guild(guild_data, &mut self.data); // Caches the guild
    }
}