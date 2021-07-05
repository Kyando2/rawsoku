use super::User;
use serde::Deserialize;

#[derive(Clone, Eq, Deserialize)]
pub struct Member {
    user: Option<User>,
}

impl Member {
    pub fn id(&self) -> String {
        self.user.as_ref().unwrap().id()
    }
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.user.as_ref().unwrap().id() == other.user.as_ref().unwrap().id()
    }
}
