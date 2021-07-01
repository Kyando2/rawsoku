use std::env::consts::OS;

use serde_json::json;

use super::CandleLight;
use crate::consts::op_code;

#[macro_export]
macro_rules! generate_intents {
    ($b:expr) => ($b);
    ( $b:expr, $( $x:expr ),* ) => {
        ($b + generate_intents!($( $x ),*))
    };
}
pub struct CandleLighter {
    payload: BuildPayload,
}

impl CandleLighter {
    pub fn new() -> CandleLighter {
        CandleLighter {
            payload: BuildPayload {
                auth_token: None,
                intents: None,
            },
        }
    }
    pub fn intents(mut self, intents: u16) -> CandleLighter {
        self.payload.intents = Some(intents);
        self
    }
    pub fn auth_token(mut self, token: &'static str) -> CandleLighter {
        self.payload.auth_token = Some(token);
        self
    }
    pub async fn light(self) -> ! {
        CandleLight::run(self.payload).await
    }
}

pub struct BuildPayload {
    auth_token: Option<&'static str>,
    intents: Option<u16>,
}

impl BuildPayload {
    pub fn get_identify_data(self) -> String {
        json!({
            "op": op_code::IDENTIFY,
            "d": {
                "token": self.auth_token.expect("Did not specify auth token"),
                "intents": self.intents.expect("Did not specify intents"),
                "properties": {
                    "$os": OS,
                    "$browser": "Rawsoku",
                    "$device": "Rawsoku",
                }
            }
        })
        .to_string()
    }
}
