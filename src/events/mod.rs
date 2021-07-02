mod cache;

pub use cache::Cache;
use serde_json::Value;

use crate::{consts::{dispatch::*, op_code::*, OnMessageHandler}, prelude::Handle, snowflakes::{Guild, Message, User}};

struct Handlers {
    on_message: Option<OnMessageHandler>,
}

pub struct Wax {
    data: Cache,
    handlers: Handlers,
    auth_token: String,
    me: Option<User>,
}

impl Wax {
    pub fn new(auth_token: String) -> Wax {
        Wax {
            data: Cache::new(),
            handlers: Handlers { on_message: None },
            auth_token,
            me: None
        }
    }
    pub fn handle(&mut self, data: Value) {
        let op_code = data["op"]
            .as_u64()
            .expect("The gateway sent a message containing an invalid op code")
            as u8;
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
            } else if data["t"] == READY {
                self.handle_ready(data);
            }
        }
    }
    pub fn set_on_message(&mut self, handler: Option<OnMessageHandler>) {
        self.handlers.on_message = handler;
    }
    fn handle_guild_create(&mut self, data: Value) {
        let guild_data = &data["d"];
        let _ = Guild::new_from_object(guild_data, &mut self.data, self.auth_token.clone());
        // Caches the guild
    }
    fn handle_ready(&mut self, data: Value) {
        let user = User::new_from_object(&data["d"]["user"], &mut self.data, self.auth_token.clone());
        self.me = Some(user);
    }
    fn handle_message(&mut self, data: Value) {
        let msg = Message::new_from_object(&data["d"], &mut self.data, self.auth_token.clone());
        if self.handlers.on_message.is_some() {
            let func = self.handlers.on_message.unwrap();
            let me = self.me.as_ref().unwrap().clone();
            std::thread::spawn(move || {
                func(Handle::new(me), msg);
            });
        }
    }
}
