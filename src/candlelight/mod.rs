mod builder;

use crate::lifestate::LifeState;
use crate::{consts, consts::op_code, get_epoch_ms, GuardedRead, GuardedWrite};
pub use builder::{BuildPayload, CandleLighter};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub struct CandleLight {
    life_state: Arc<Mutex<LifeState>>,
}

impl CandleLight {
    pub async fn run(payload: BuildPayload) -> ! {
        let (client, response) = connect_async(Url::parse(consts::GATEWAY).unwrap())
            .await
            .expect("Can't connect");

        let (write, mut read) = client.split();
        let hello_message = read
            .next()
            .await
            .expect("Couldn't read hello message")
            .unwrap();
        let hello_json: serde_json::Value = serde_json::from_str(hello_message.to_text().unwrap())
            .expect("Could not parse as JSON");
        let life_state = LifeState::new(Duration::from_millis(
            hello_json["d"]["heartbeat_interval"]
                .as_u64()
                .expect("Could not get interval"),
        ));
        //{
        //  interval: Duration::from_millis(hello_json["d"]["heartbeat_interval"].as_u64().expect("Could not get interval")),
        //sequence: 0,
        // last_beat: get_epoch_ms(),
        //};
        let light = CandleLight {
            life_state: Arc::new(Mutex::new(life_state)),
        };
        let read_guard = Arc::new(Mutex::new(read));
        let write_guard = Arc::new(Mutex::new(write));
        light.identify(write_guard.clone(), payload).await;
        light
            .start_dispatcher(read_guard.clone(), write_guard.clone())
            .await
    }
    async fn identify(&self, write: GuardedWrite, payload: BuildPayload) {
        // throw away identify
        let identify_payload = payload.get_identify_data();
        write
            .lock()
            .unwrap()
            .send(Message::text(identify_payload))
            .await;
    }
    async fn start_dispatcher(&self, reader: GuardedRead, write: GuardedWrite) -> ! {
        let interval = self.life_state.lock().unwrap().interval();
        loop {
            match tokio::time::timeout(
                Duration::from_millis(5000u64),
                reader.lock().unwrap().next(),
            )
            .await
            {
                Ok(val) => {
                    println!(
                        "{}",
                        val.expect("Could not read")
                            .expect("Could not read")
                            .to_text()
                            .unwrap()
                    );
                }
                Err(_) => {} // Do nothing not a problem, there simply wasn't a new message
            }
            if self.life_state.lock().unwrap().last_beat() + interval.as_millis() <= get_epoch_ms()
            {
                write
                    .lock()
                    .unwrap()
                    .send(Message::text(
                        self.life_state.lock().unwrap().heartbeat_payload(),
                    ))
                    .await;
                self.life_state.lock().unwrap().heartbeat_now();
            }
        }
    }
}
