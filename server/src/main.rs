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
    let mut id: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

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
                println!("disconnected, number of clients: {}", clients.len());
            }
        }
    });

    loop {
        let mut buf = [0; 1024];
        let socket = socket.clone(); // Clone the socket
        let (n, addr) = socket.recv_from(&mut buf).await?;
        let id = id.clone();

        let clients_ref = clients.clone();
        tokio::spawn(async move {
            match buf[0] {
                PING => {
                    // println!("received ping from client {}", addr);
                    let _ = socket.send_to(&buf[0..n], &addr).await;
                }
                UPDATE_POSITION if n >= 10 => {
                    let pos_x = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
                    let pos_y = f32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]);
                    let level = buf[9];
                    // println!("received pos ({},{}) from client {}", pos_x, pos_y, addr);
                    let mut taskid = id.lock().await;
                    let user_id = *taskid;

                    let mut clients = clients_ref.lock().await;
                    let client_count = clients.len();
                    let client = clients.entry(addr).or_insert(Client {
                        addr,
                        pos_x,
                        pos_y,
                        level,
                        user_id,
                        last_message: Instant::now(),
                    });

                    client.pos_x = pos_x;
                    client.pos_y = pos_y;
                    client.last_message = Instant::now();
                    client.level = level;
                    let user_id = client.user_id;

                    // Send positions to other clients
                    let mut buf = [UPDATE_POSITION, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    let bytes = pos_x.to_le_bytes();
                    buf[1..5].copy_from_slice(&bytes);
                    let bytes = pos_y.to_le_bytes();
                    buf[5..9].copy_from_slice(&bytes);
                    let bytes = user_id.to_le_bytes();
                    buf[9..13].copy_from_slice(&bytes);

                    for other_client in clients.values() {
                        // should divide clients based on level
                        let same_level = other_client.level == level;
                        if same_level && other_client.addr != addr {
                            // println!("sending pos ({},{}) to client {}", pos_x, pos_y, other_client.addr);
                            let _ = socket.send_to(&buf, &other_client.addr).await;
                        }
                    }

                    let new_client_count = clients.len();
                    if new_client_count != client_count {
                        println!("connected, number of clients: {}", new_client_count);
                        // increase id value
                        *taskid += 1;
                        drop(taskid);
                    }
                }
                _ => {} // Ignore other cases
            }
        });
    }
}

struct Client {
    addr: SocketAddr,
    pos_x: f32,
    pos_y: f32,
    level: u8,
    user_id: u32,
    last_message: Instant,
}
