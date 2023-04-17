mod handler;
mod cleaner;
mod spawner;
mod messages;

use handler::handle_message;
use cleaner::remove_inactive_clients;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:1234").await?);
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let entities: Arc<Mutex<HashMap<u32, Entity>>>  = Arc::new(Mutex::new(HashMap::new()));
    let id = Arc::new(Mutex::new(0));

    // Spawn the cleanup task
    let clients_ref = clients.clone();
    tokio::spawn(async move {
        remove_inactive_clients(clients_ref).await;
    });
    // Spawn the spawner task
    let clients_ref = clients.clone();
    let entities_ref = entities.clone();
    tokio::spawn(async move {
        spawner::spawn_entities(clients_ref, entities_ref).await;
    });

    loop {
        let mut buf = [0; 1024];
        let socket = socket.clone(); // Clone the socket
        let (n, addr) = socket.recv_from(&mut buf).await?;
        let id = id.clone();

        let clients_ref = clients.clone();
        let entities_ref = entities.clone();
        tokio::spawn(async move {
            handle_message(buf, n, socket, addr, id, clients_ref, entities_ref).await;
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

#[derive(Clone)]
struct Entity {
    pos_x: f32,
    pos_y: f32,
    level: u8,
    id: u32,
    user_id: u32,
    health: u8,
}