use crate::structs::{Client,Entity};
use crate::handler::FOLLOW;
use crate::handler::UNFOLLOW;

use tokio::{net::UdpSocket, sync::Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

pub(crate) async fn handle_follow(
    buf: [u8; 1024],
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    entities: Arc<Mutex<HashMap<u32, Entity>>>,
) {
    let clients = clients.lock().await;
    let mut entities = entities.lock().await;

    let entity_id = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
    let entity = entities.get_mut(&entity_id).unwrap();
    let client = clients.get(&addr).unwrap();


    //fond previous client address
    let prev_user_id = entity.user_id;
    let prev_client = clients.iter().find(|(_, c)| c.user_id == prev_user_id).unwrap().1;
    // send unfollow to the previous client
    let mut buf = [0; 6];
    buf[0] = UNFOLLOW;
    buf[1..5].copy_from_slice(&entity_id.to_be_bytes());
    socket.send_to(&buf, prev_client.addr).await.unwrap();

    entity.user_id = client.user_id;

    drop(clients);
    drop(entities);

    //inform the client the entity is following
    let mut buf = [0; 6];
    buf[0] = FOLLOW;
    buf[1..5].copy_from_slice(&entity_id.to_be_bytes());

    socket.send_to(&buf, addr).await.unwrap();
}