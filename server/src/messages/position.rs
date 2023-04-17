use crate::Client;
use crate::handler::UPDATE_POSITION;

use tokio::{net::UdpSocket, sync::Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;

pub(crate) async fn handle_update_position(
    buf: [u8; 1024],
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
    id: Arc<Mutex<u32>>,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
) {
    let pos_x = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
    let pos_y = f32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]);
    let level = buf[9];
    // println!("received pos ({},{}) from client {}", pos_x, pos_y, addr);
    let taskid = id.lock().await;
    let user_id = *taskid;
    drop(taskid);

    let mut clients_values = clients.lock().await;
    let client_count = clients_values.len();
    let client = clients_values.entry(addr).or_insert(Client {
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
    drop(clients_values);
    let clients_copy = clients.clone();
    send_update_position(socket, clients_copy, addr, pos_x, pos_y, level, user_id, id, client_count).await;
}

async fn send_update_position(
    socket: Arc<UdpSocket>,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    addr: SocketAddr,
    pos_x: f32,
    pos_y: f32,
    level: u8,
    user_id: u32,
    id: Arc<Mutex<u32>>,
    old_client_count: usize,
) {
    // Send positions to other clients
    let mut buf = [UPDATE_POSITION, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let bytes = pos_x.to_le_bytes();
    buf[1..5].copy_from_slice(&bytes);
    let bytes = pos_y.to_le_bytes();
    buf[5..9].copy_from_slice(&bytes);
    let bytes = user_id.to_le_bytes();
    buf[9..13].copy_from_slice(&bytes);

    let clients = clients.lock().await;
    for other_client in clients.values() {
        // should divide clients based on level
        let same_level = other_client.level == level;
        if same_level && other_client.addr != addr {
            // println!("sending pos ({},{}) to client {}", pos_x, pos_y, other_client.addr);
            let _ = socket.send_to(&buf, &other_client.addr).await;
        }
    }

    let new_client_count = clients.len();
    if new_client_count != old_client_count {
        println!("connected, number of clients: {}", new_client_count);
        // increase id value
        let mut taskid = id.lock().await;
        *taskid += 1;
        drop(taskid);
    }
}