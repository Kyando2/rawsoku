use rawsoku::generate_intents;
use rawsoku::prelude::*;
use tokio;

#[tokio::test]
async fn basic_test() {
    CandleLighter::new()
        .intents(generate_intents!(GUILDS, GUILD_MESSAGES))
        .auth_token("No :)")
        .on_message(omh)
        .light()
        .await;
}

fn omh(h: Handle, c: Channel) {
    c.send_message(h, "Hello".to_string());
}