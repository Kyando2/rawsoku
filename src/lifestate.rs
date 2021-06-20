use std::time::Duration;
use serde_json::json;

use crate::consts;
use crate::consts::OpCode;

use crate::get_epoch_ms;

pub struct LifeState {
    interval: Duration,
    sequence: u32,
    last_beat: u128,
}

impl LifeState {
    pub fn new(interval: Duration) -> Self {
        LifeState {
            interval,
            sequence: 0,
            last_beat: get_epoch_ms()
        }
    }
    pub fn heartbeat_payload(&self) -> String {
        if self.sequence == 0 {
            json!({
                "op": OpCode::HEARTBEAT,
                "d": "null",
            }).to_string()
        } else {
            json!({
                "op": OpCode::HEARTBEAT,
                "d": self.sequence,
            }).to_string()
        }
    }
    pub fn update_sequence(&mut self, seq: u32) {
        self.sequence = seq;
    }
    pub fn sequence(&self) -> u32 {
        self.sequence
    }
    pub fn last_beat(&self) -> u128 {
        self.last_beat
    }
    pub fn interval(&self) -> Duration {
        self.interval
    }
    pub fn heartbeat_now(&mut self) {
        self.last_beat = get_epoch_ms();
    }
}