use tokio;
use tokio_tungstenite;
pub use tungstenite::protocol::Message;
use tungstenite::protocol::frame::Utf8Bytes;
use tungstenite::Bytes;
use std::thread;
use tokio::net::TcpListener;

use futures_util::{StreamExt, SinkExt};
use std::sync::mpsc;

pub struct Connection {
    pub send_to_client: mpsc::Sender<Message>,
    pub receive_from_client: mpsc::Receiver<Message>
}

impl Connection {
    pub fn send_text(&self, string: &str) {
        self.send_to_client.send(Message::text(string)).unwrap();
    }
}
pub fn run(port: i32) -> mpsc::Receiver<Connection> {

    let (send_new_connection, receive_new_connections) = mpsc::channel::<Connection>();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to start tokio runtime");
        rt.block_on(async {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.expect("Failed to bind to port");
            loop {
                let (stream, _socket_addr) = listener.accept().await.expect("Failed to accept tcp stream");
                let stream = tokio_tungstenite::accept_async(stream).await.expect("Failed to init websocket connection");

                let (mut write_stream, mut read_stream) = stream.split();

                let (send_to_main_thread, receive_from_client) = mpsc::channel::<Message>();
                let (send_to_client, send_to_client_transmitter) = mpsc::channel::<Message>();
                
                let send_to_client_reference = send_to_client.clone();
                //listener
                tokio::spawn(async move {
                    loop {
                        let message = read_stream.next().await.expect("Failed to get message").expect("Failed to unpack message");
                        match message {
                            Message::Text(text) => {
                                send_to_main_thread.send(Message::Text(text));
                                //let string = text.as_str().to_owned();
                                
                               // println!("{}", string);
                            },
                            Message::Ping(_bytes) => {
                                send_to_client_reference.send(Message::Pong(Bytes::from("Pong from server!!!!"))).expect("Failed to send to send thread");
                            },
                            _ => {
                                println!("Unrecognised Data format");
                            }
                        }
                    }
                });

                //sender
                tokio::spawn(async move {
                    loop {
                        let message: Message = send_to_client_transmitter.recv().unwrap();
                        write_stream.send(message).await.expect("Failed to send message");
                    }
                });


                let connection: Connection = Connection {
                    send_to_client,
                    receive_from_client
                };
                send_new_connection.send(connection).expect("Failed to send connection to main thread");
            }
        });
    });
    return receive_new_connections;
}