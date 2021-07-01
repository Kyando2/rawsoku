use rawsoku::prelude::*;
use rawsoku::generate_intents;
use tokio;

#[tokio::test]
async fn basic_test() {
    CandleLighter::new()
        .intents(generate_intents!(GUILDS, GUILD_MESSAGES))
        .auth_token("No. :)")
        .light()
        .await;
}
