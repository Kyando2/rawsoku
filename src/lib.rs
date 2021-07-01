#![warn(clippy::pedantic)]

mod candlelight;
mod consts;
mod lifestate;
pub mod prelude;

use std::{
    io::stdin,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use serde_json::json;
use tokio;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task,
};
use tokio_native_tls;
use tokio_tungstenite::{
    connect_async, tungstenite::client::AutoStream, tungstenite::protocol::Message, WebSocketStream,
};
use url::Url;

// Internals
use candlelight::CandleLight;
use lifestate::LifeState;

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
