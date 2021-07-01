use rawsoku::generate_intents;
use rawsoku::prelude::*;
use tokio;

#[tokio::test]
async fn basic_test() {
    CandleLighter::new()
        .intents(generate_intents!(GUILDS, GUILD_MESSAGES))
        .auth_token("ODMwMjA4MDEyNjY4NzY0MjUw.YHDVdg.va4E7XiHK-N1EvNFU5vPGkyKWZc")
        .light()
        .await;
}
