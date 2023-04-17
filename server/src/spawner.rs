use std::collections::HashMap;
use std::{time::Duration, sync::Arc};
use tokio::sync::Mutex;
use std::net::SocketAddr;

use tokio::time::interval;

use crate::{Client, Entity};

fn shift_pos_to_inbound(x: i32, y: i32, level: u8) -> (i32, i32) {
    
    // TODO: implement this function

    (0,0)
}

pub(crate) async fn spawn_entities(
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    entities: Arc<Mutex<HashMap<u32, Entity>>>
) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        let total_entities = entities.lock().await.len();
        if total_entities < 100 {
            let rand_x = 0;
            let rand_y = 0;
            let (rand_x, rand_y) = shift_pos_to_inbound(rand_x, rand_y, 0);

            let mut entities_ref = entities.lock().await;
            let new_entity = entities_ref.entry(0/* fix me */).or_insert(Entity {
                pos_x: rand_x as f32,
                pos_y: rand_y as f32,
                level: 0,
                id: 0, // fix me
                user_id: 0,
                health: 100,
            });

            // inform all clients about the new entity
            send_entity_to_clients(new_entity.clone(), clients.clone());
        }
    }
}

fn send_entity_to_clients(entity: Entity, clients: Arc<Mutex<HashMap<SocketAddr, Client>>>) {

    // TODO: implement this function

}