use tokio::{net::UdpSocket, sync::Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

use crate::{Client, Entity};
use crate::messages::ping::handle_ping;
use crate::messages::position::handle_update_position;

pub const PING: u8 = 0x01;
pub const UPDATE_POSITION: u8 = 0x02;
// pub const ENTITY_SPAWN: u8 = 0x03;
// pub const ENTITY_FOLLOW: u8 = 0x04;
// pub const ENTITY_UNFOLLOW: u8 = 0x05;
// pub const ENTITY_MOVE: u8 = 0x06;
// pub const ENTITY_HURT: u8 = 0x07;
// pub const ENTITY_DESTROY: u8 = 0x08;

pub(crate) async fn handle_message(
    buf: [u8;1024], 
    msg_len: usize, 
    socket: Arc<UdpSocket>, 
    addr: SocketAddr, 
    id: Arc<Mutex<u32>>,
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    entities: Arc<Mutex<HashMap<u32, Entity>>>,
) {
    match buf[0] {
        PING => {
            handle_ping(buf, msg_len, socket, addr).await;
        }
        UPDATE_POSITION if msg_len >= 10 => {
            handle_update_position(buf, socket, addr, id, clients).await;
        }
        _ => {} // Ignore other cases
    }
}