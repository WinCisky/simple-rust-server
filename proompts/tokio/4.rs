// I need to make and udp server in rust using tokio that's as performant as possible. 
// When the server receives a message from the client it needs to parse the first byte of the request and if the first byte in 
// 0x01 then it send an empty message to the client that sent the message to the server (ping) otherwise if the first byte is 
// 0x02 then the server stores bytes 1 to 4 as posx and 5 to 9 as pos y, the server stores the timestamp of the client last message. 
// In another asyncronous task the server checks if the last timestamp of every client is older than 5s, if so removes it from the clients list, 
// subsequently sends every player position to every client except himself.

// To create a performant UDP server in Rust using Tokio, you can use the tokio::net::UdpSocket struct for handling the UDP socket and tokio::spawn to spawn the concurrent tasks. 
// Here's an example implementation based on your requirements:

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::interval;

const PING: u8 = 0x01;
const UPDATE_POSITION: u8 = 0x02;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let clients = Mutex::new(HashMap::new());

    // Spawn the cleanup task
    let clients_ref = clients.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            let mut clients = clients_ref.lock().await;
            clients.retain(|_, client| client.last_message.elapsed() < Duration::from_secs(5));
        }
    });

    loop {
        let mut buf = [0; 1024];
        let (n, addr) = socket.recv_from(&mut buf).await?;

        let clients_ref = clients.clone();
        tokio::spawn(async move {
            match buf[0] {
                PING => {
                    let _ = socket.send_to(&buf[0..n], &addr).await;
                }
                UPDATE_POSITION if n >= 10 => {
                    let pos_x = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
                    let pos_y = u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]);

                    let mut clients = clients_ref.lock().await;
                    let client = clients.entry(addr).or_insert(Client {
                        addr,
                        pos_x,
                        pos_y,
                        last_message: Instant::now(),
                    });

                    client.pos_x = pos_x;
                    client.pos_y = pos_y;
                    client.last_message = Instant::now();

                    // Send positions to other clients
                    for other_client in clients.values() {
                        if other_client.addr != addr {
                            let mut buf = [UPDATE_POSITION, 0, 0, 0, 0, 0, 0, 0, 0];
                            buf[1..5].copy_from_slice(&other_client.pos_x.to_be_bytes());
                            buf[5..9].copy_from_slice(&other_client.pos_y.to_be_bytes());

                            let _ = socket.send_to(&buf, &addr).await;
                        }
                    }
                }
                _ => {}
            }
        });
    }
}

struct Client {
    addr: SocketAddr,
    pos_x: u32,
    pos_y: u32,
    last_message: Instant,
}


// In this example, we create a performant UDP server using Tokio and handle different message types based on the first byte. 
// We also store the client information in a shared HashMap and spawn a separate cleanup task that removes clients with outdated timestamps. 
// Finally, we send the position updates to all clients except the sender.