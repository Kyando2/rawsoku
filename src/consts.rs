pub const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=9&encoding=json";
pub const BASE_URL: &'static str = "https://discord.com/api";

pub mod op_code {
    pub const DISPATCH: u8 = 0;
    pub const HEARTBEAT: u8 = 1;
    pub const IDENTIFY: u8 = 2;
    pub const PRESENCE_UPDATE: u8 = 3;
    pub const VOICE_STATE_UPDATE: u8 = 4;
    pub const RESUME: u8 = 6;
    pub const RECONNECT: u8 = 7;
    pub const REQUEST_GUILD_MEMBERS: u8 = 8;
    pub const INVALID_SESSION: u8 = 9;
    pub const HELLO: u8 = 10;
    pub const HEARTBEAT_ACK: u8 = 11;
}

pub mod dispatch {
    pub const GUILD_CREATE: &'static str = "GUILD_CREATE";
    pub const MESSAGE_CREATE: &'static str = "MESSAGE_CREATE";
}

pub mod intents {
    pub const GUILDS: u16 = 1 << 0;
    pub const GUILD_MEMBERS: u16 = 1 << 1;
    pub const GUILD_BANS: u16 = 1 << 2;
    pub const GUILD_EMOJIS: u16 = 1 << 3;
    pub const GUILD_INTEGRATIONS: u16 = 1 << 4;
    pub const GUILD_WEBHOOKS: u16 = 1 << 5;
    pub const GUILD_INVITES: u16 = 1 << 6;
    pub const GUILD_VOICE_STATES: u16 = 1 << 7;
    pub const GUILD_PRESENCES: u16 = 1 << 8;
    pub const GUILD_MESSAGES: u16 = 1 << 9;
    pub const GUILD_MESSAGE_REACTIONS: u16 = 1 << 10;
    pub const GUILD_MESSAGE_TYPING: u16 = 1 << 11;
    pub const DIRECT_MESSAGES: u16 = 1 << 12;
    pub const DIRECT_MESSAGE_REACTIONS: u16 = 1 << 13;
    pub const DIRECT_MESSAGE_TYPING: u16 = 1 << 14;
}
