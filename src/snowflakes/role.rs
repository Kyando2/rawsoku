use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Role {
    id: String,
    name: String,
    color: u32,
    hoist: bool,
    permissions: String,
    managed: bool,
    mentionable: bool,
}

impl Role {}
