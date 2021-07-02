use serde_json::Value;

#[derive(Clone)]
pub struct Role;

impl Role {
    pub fn new_from_object(data: &Value) -> Role {
        println!("Role data: {}", data.to_string());
        Role {}
    }
}
