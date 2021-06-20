mod lifestate;
mod consts;
mod candlelight;

// Std
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Mutex, Arc};
use std::thread;
use std::sync::mpsc::channel;
use std::io::stdin;

// Externals
use serde_json::json;
use futures_util::{future, pin_mut, StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, tungstenite::client::AutoStream, WebSocketStream};
use url::Url;
use futures_util::stream::{SplitStream, SplitSink};
use tokio;
use tokio::{task, net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use tokio_native_tls;

// Internals
use lifestate::LifeState;

// Exports
pub use candlelight::CandleLight;

pub type GuardedRead = Arc<std::sync::Mutex<SplitStream<WebSocketStream<tokio_tungstenite::stream::Stream<tokio::net::TcpStream, tokio_native_tls::TlsStream<tokio::net::TcpStream>>>>>>;
pub type GuardedWrite = Arc<std::sync::Mutex<SplitSink<WebSocketStream<tokio_tungstenite::stream::Stream<tokio::net::TcpStream, tokio_native_tls::TlsStream<tokio::net::TcpStream>>>, Message>>>;


fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}


