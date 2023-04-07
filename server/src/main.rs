use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::interval;

const PING: u8 = 0x01;
const UPDATE_POSITION: u8 = 0x02;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:8080").await?);
    let clients: Arc<Mutex<HashMap<SocketAddr, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    // Spawn the cleanup task
    let clients_ref = clients.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            let mut clients = clients_ref.lock().await;
            let clients_number = clients.len();
            clients.retain(|_, client| client.last_message.elapsed() < Duration::from_secs(5));
            if clients.len() != clients_number {
                println!("clients disconnected, number of clients: {}", clients.len());
            }
        }
    });

    loop {
        let mut buf = [0; 1024];
        let socket = socket.clone(); // Clone the socket
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
                _ => {} // Ignore other cases
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
