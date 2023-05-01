use std::collections::HashMap;
use std::{time::Duration, sync::Arc};
use tokio::sync::Mutex;
use std::net::SocketAddr;

use tokio::time::interval;

use crate::structs::Client;

pub(crate) async fn remove_inactive_clients(clients: Arc<Mutex<HashMap<SocketAddr, Client>>>) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        let mut clients_ref = clients.lock().await;
        let clients_number = clients_ref.len();
        clients_ref.retain(|_, client| client.last_message.elapsed() < Duration::from_secs(5));
        if clients_ref.len() != clients_number {
            println!("disconnected, number of clients: {}", clients_ref.len());
        }
    }
}