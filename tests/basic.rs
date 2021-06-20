use tokio;
use rawsoku;

#[tokio::test]
async fn basic_test() {
    rawsoku::CandleLight::run().await;
}