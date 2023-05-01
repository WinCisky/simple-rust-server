mod handler;
mod cleaner;
mod spawner;
mod messages;
mod structs;

use handler::handle_message;
use cleaner::remove_inactive_clients;
use structs::Entity;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:1234").await?);
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let entities: Arc<Mutex<HashMap<u32, Entity>>>  = Arc::new(Mutex::new(HashMap::new()));
    let user_id = Arc::new(Mutex::new(1));
    let entities_id = Arc::new(Mutex::new(1));

    // Spawn the cleanup task
    let clients_ref = clients.clone();
    tokio::spawn(async move {
        remove_inactive_clients(clients_ref).await;
    });
    // Spawn the spawner task
    let clients_ref = clients.clone();
    let entities_ref = entities.clone();
    let socket_ref = socket.clone();
    tokio::spawn(async move {
        spawner::spawn_entities(socket_ref, clients_ref, entities_ref, entities_id).await;
    });

    loop {
        let mut buf = [0; 1024];
        let (n, addr) = socket.recv_from(&mut buf).await?;

        let clients_ref = clients.clone();
        let entities_ref = entities.clone();
        let userid_ref = user_id.clone();
        let sockert_ref = socket.clone();
        tokio::spawn(async move {
            handle_message(buf, n, sockert_ref, addr, userid_ref, clients_ref, entities_ref).await;
        });
    }
}