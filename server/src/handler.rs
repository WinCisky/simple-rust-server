use tokio::{net::UdpSocket, sync::Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;

use crate::Client;

const PING: u8 = 0x01;
const UPDATE_POSITION: u8 = 0x02;

pub(crate) async fn handle_message(
    buf: [u8;1024], 
    msg_len: usize, 
    socket: Arc<UdpSocket>, 
    addr: SocketAddr, 
    id: Arc<Mutex<u32>>,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>
) {
    match buf[0] {
        PING => {
            // println!("received ping from client {}", addr);
            let _ = socket.send_to(&buf[0..msg_len], &addr).await;
        }
        UPDATE_POSITION if msg_len >= 10 => {
            let pos_x = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
            let pos_y = f32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]);
            let level = buf[9];
            // println!("received pos ({},{}) from client {}", pos_x, pos_y, addr);
            let mut taskid = id.lock().await;
            let user_id = *taskid;

            let mut clients = clients.lock().await;
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
}