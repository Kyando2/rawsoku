mod cache;

use std::{collections::HashMap, sync::{Arc, Mutex}};

pub use cache::Cache;
use serde_json::Value;

use crate::{consts::{dispatch::*, op_code::*, OnMessageHandler}, lifestate::LifeState, prelude::Handle, snowflakes::{Guild, Message, User}};

struct Handlers {
    on_message: Option<OnMessageHandler>,
    commands: Vec<(String, fn(Context))>
}

pub struct Context {
    args: HashMap<String, String>,
    handle: Handle
}

pub struct Wax {
    data: Cache,
    handlers: Handlers,
    auth_token: String,
    me: Option<User>,
}

impl Wax {
    pub fn new(auth_token: String) -> Wax {
        let mut wax = Wax {
            data: Cache::new(),
            handlers: Handlers { on_message: None, commands: Vec::new() },
            auth_token,
            me: None,
        };
        wax.gen_commands();
        wax
    }
    pub fn handle(&mut self, data: Value, lifestate: Arc<Mutex<LifeState>>) {
        let op_code = data["op"]
            .as_u64()
            .expect("The gateway sent a message containing an invalid op code")
            as u8;
        if op_code == HEARTBEAT_ACK {
            lifestate.lock().unwrap().update_sequence(data["s"].as_u64().unwrap_or(0) as u32);
        } else if op_code == DISPATCH {
            if data["t"] == GUILD_CREATE {
                self.handle_guild_create(data);
            } else if data["t"] == MESSAGE_CREATE {
                self.handle_message(data);
            } else if data["t"] == READY {
                self.handle_ready(data);
            }
        }
    }
    pub fn set_on_message(&mut self, handler: Option<OnMessageHandler>) {
        self.handlers.on_message = handler;
    }
    fn handle_guild_create(&mut self, data: Value) {
        let guild_data = data["d"].clone();
        let g: Guild = serde_json::from_value(guild_data).unwrap();
        self.data.new_guild(g);
    }
    fn handle_ready(&mut self, data: Value) {
        let user: User = serde_json::from_value(data["d"]["user"].clone()).unwrap();
        self.me = Some(user);
    }
    fn gen_commands(&mut self) {}
    fn handle_message(&mut self, data: Value) {
        let msg: Message = serde_json::from_value(data["d"].clone()).unwrap();
        if self.handlers.on_message.is_some() {
            let func = self.handlers.on_message.unwrap();
            let me = self.me.as_ref().unwrap().clone();
            let auth = self.auth_token.clone();
            std::thread::spawn(move || {
                func(Handle::new(me, auth), msg);
            });
        }
    }
}
