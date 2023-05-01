use std::collections::HashMap;
use std::{time::Duration, sync::Arc};
use tokio::sync::Mutex;
use std::net::SocketAddr;
use rand::Rng;
use tokio::net::UdpSocket;
use tokio::time::interval;

use crate::structs::{Client, Entity};

fn shift_pos_to_inbound(x: i32, y: i32, _level: u8) -> (i32, i32) {
    
    // TODO: implement this function

    (x,y)
}

pub(crate) async fn spawn_entities(
    socket: Arc<UdpSocket>,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    entities: Arc<Mutex<HashMap<u32, Entity>>>,
    id: Arc<Mutex<u32>>,
) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        let total_entities = entities.lock().await.len();
        if total_entities >= 100 {
            continue;
        }
        println!("spawning new entity");
        let rand_x = rand::thread_rng().gen_range(-100..100);
        let rand_y = rand::thread_rng().gen_range(-100..100);
        let (rand_x, rand_y) = shift_pos_to_inbound(rand_x, rand_y, 0);

        let mut index = id.lock().await;
        let index_value = *index;            

        let mut entities_ref = entities.lock().await;
        let new_entity = entities_ref.entry(index_value).or_insert(Entity {
            pos_x: rand_x as f32,
            pos_y: rand_y as f32,
            level: 0,
            id: index_value,
            user_id: 0,
            health: 100,
        });

        *index += 1;
        drop(index);

        // inform all clients about the new entity
        send_entity_to_clients(socket.clone(), new_entity.clone(), clients.clone()).await;
    }
}

// send a move message to every client
async fn send_entity_to_clients(
    socket: Arc<UdpSocket>,
    entity: Entity,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>
) {

    let message = entity.format_message_packet_movement();

    let clients = clients.lock().await;
    for other_client in clients.values() {
        // should divide clients based on level
        let same_level = true;
        if same_level {
            let _ = socket.send_to(&message, &other_client.addr).await;
        }
    }

}