use std::str;

use serde_json::Value;

use crate::events::Cache;

#[derive(Clone, Eq)]
pub struct User {
    id: String,
    auth: String,
}

impl User {
    pub fn new_from_object(data: &Value, cache: &mut Cache, auth: String) -> User {
        dbg!(&data);
        let id = data["id"].as_str().unwrap().to_string();
        let user = User {
            id,
            auth,
        };
        cache.new_user(user.clone());
        user
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
    }
}