use rust_ws_gameserver;
use rust_ws_gameserver::{Connection, Message};
use std::thread;

const TICK_RATE: f64 = 5.0;
#[test]
fn test1 () {
    let listener = rust_ws_gameserver::run(3000);
    let mut connections: Vec<Connection> = Vec::new();

    loop {
        //load new connections
        loop {
            match listener.try_recv() {
                Ok(connection) => {
                    connections.push(connection);
                }
                _ => {
                    break;
                } 
            }
        }

        //listen to client messages
        for i in 0..connections.len() {
            let connection = &connections[i];
            loop {
                let message = match connection.receive_from_client.try_recv() {
                    Ok(message) => {
                        message
                    }
                    _ => {
                        break;
                    }
                };
                match message {
                    Message::Text(text) => {
                        let string = text.as_str().to_owned();
                        connection.send_text("Hello from server!");
                        println!("{}", string);
                    }
                    _ => {
                        println!("Received not valid message")
                    }
                }
            }
        }
        thread::sleep(std::time::Duration::from_micros((1_000_000.0 / TICK_RATE) as u64));
    }
}