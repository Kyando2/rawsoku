use serde_json::Value;

#[derive(Clone)]
pub struct User;

impl User {
    pub fn gen_create_guild(data: &Value) -> User {
        println!("User data: {}", data.to_string());
        User {}
    }
}