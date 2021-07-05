use serde::Deserialize;

#[derive(Clone, Eq, Deserialize)]
pub struct User {
    id: String,
    auth: Option<String>,
}

impl User {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
