#![warn(clippy::pedantic)]

mod candlelight;
mod consts;
mod events;
mod http;
mod lifestate;
pub mod prelude;
pub mod snowflakes;

use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::stream::{SplitSink, SplitStream};
use tokio;
use tokio_native_tls;
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};

pub type GuardedRead = Arc<
    Mutex<
        SplitStream<
            WebSocketStream<
                tokio_tungstenite::stream::Stream<
                    tokio::net::TcpStream,
                    tokio_native_tls::TlsStream<tokio::net::TcpStream>,
                >,
            >,
        >,
    >,
>;
pub type GuardedWrite = Arc<
    Mutex<
        SplitSink<
            WebSocketStream<
                tokio_tungstenite::stream::Stream<
                    tokio::net::TcpStream,
                    tokio_native_tls::TlsStream<tokio::net::TcpStream>,
                >,
            >,
            Message,
        >,
    >,
>;

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
