use rawsoku::generate_intents;
use rawsoku::prelude::*;
use tokio;

#[tokio::test]
async fn basic_test() {
    CandleLighter::new()
        .intents(generate_intents!(GUILDS, GUILD_MESSAGES))
        .auth_token("No. :)")
        .light()
        .await;
}
