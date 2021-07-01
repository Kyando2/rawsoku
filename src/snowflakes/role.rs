use serde_json::Value;

#[derive(Clone)]
pub struct Role;

impl Role {
    pub fn gen_create_guild(data: &Value) -> Role {
        println!("Role data: {}", data.to_string());
        Role {}
    }
}