use crate::lifestate::LifeState;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::{StreamExt, SinkExt};
use std::time::Duration;
use crate::{GuardedWrite, GuardedRead, get_epoch_ms, consts, consts::OpCode};
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

pub struct CandleLight {
    life_state: Arc<Mutex<LifeState>>,
}

impl CandleLight {
    pub async fn run() {
        let (client, response) =
            connect_async(Url::parse(consts::GATEWAY).unwrap()).await.expect("Can't connect");

        let (write, mut read) = client.split();
        let hello_message = read.next().await.expect("Couldn't read hello message").unwrap();
        let hello_json: serde_json::Value = serde_json::from_str(hello_message.to_text().unwrap()).expect("Could not parse as JSON");
        let life_state = LifeState::new(Duration::from_millis(hello_json["d"]["heartbeat_interval"].as_u64().expect("Could not get interval")));
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
        light.identify(write_guard.clone()).await;
        light.start_dispatcher(read_guard.clone(), write_guard.clone()).await;
    }
    async fn identify(&self, write: GuardedWrite) {
        // throw away identify
        let identify_payload = json!({
            "op": OpCode::IDENTIFY,
            "d": {
                "token": "NO", // For testing
                "intents": 513,
                "properties": {
                    "$os": "windows",
                    "$browser": "Rawsoku",
                    "$device": "Rawsoku",
                }
            }
        });
        write.lock().unwrap().send(Message::text(identify_payload.to_string())).await;
    }
    async fn start_dispatcher(&self, reader: GuardedRead, write: GuardedWrite) -> ! {
        let interval =self.life_state.lock().unwrap().interval();
        loop {
            match tokio::time::timeout(Duration::from_millis(5000u64),reader.lock().unwrap().next()).await {
                Ok(val) => {
                    println!("{}", val.expect("Could not read").expect("Could not read").to_text().unwrap());
                },
                Err(_) => {} // Do nothing not a problem, there simply wasn't a new message
            }
            if self.life_state.lock().unwrap().last_beat() + interval.as_millis() <= get_epoch_ms() {
                write.lock().unwrap().send(Message::text(self.life_state.lock().unwrap().heartbeat_payload())).await;
                self.life_state.lock().unwrap().heartbeat_now();
            }
        }
    }
}