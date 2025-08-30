use tokio;
use tokio_tungstenite;
use std::thread;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use std::sync::mpsc;

pub enum MessageType {
    
}
pub struct Message<T> {
    message_type: MessageType,
    data: T
}
pub struct Connection {
    
}
pub fn run(port: i32) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to start tokio runtime");
        rt.block_on(async {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.expect("Failed to bind to port");
            let (stream, _socket_addr) = listener.accept().await.expect("Failed to accept tcp stream");
            let stream = tokio_tungstenite::accept_async(stream).await.expect("Failed to init websocket connection");

            let (read_stream, write_stream) = stream.split();

            let (ws_thread_read, main_thread_write) = mpsc::channel::<Message<i32>>();
            let (main_thread_read, ws_thread_write) = mpsc::channel::<Message<i32>>();

            tokio::spawn(async {
                loop {
                    read_stream.poll_read();
                }
            });
        });
    });
}