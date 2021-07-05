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

fn omh(h: Handle, c: Message) {
    if h.me() != c.author() {
        c.reply(h, c.content().to_string());
    }
}
