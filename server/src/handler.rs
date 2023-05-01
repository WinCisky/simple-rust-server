use tokio::{net::UdpSocket, sync::Mutex};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

use crate::structs::{Client, Entity};
use crate::messages::ping::handle_ping;
use crate::messages::position::handle_move;
use crate::messages::entities::handle_follow;

pub const PING: u8 = 0x01;
pub const MOVE: u8 = 0x02;
pub const FOLLOW: u8 = 0x3;
pub const UNFOLLOW: u8 = 0x04;
pub const ATTACK: u8 = 0x05; //animation
pub const HIT: u8 = 0x06;
pub const DIE: u8 = 0x07;

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
        MOVE => {
            handle_move(buf, socket, addr, id, clients).await;
        }
        FOLLOW => {
            handle_follow(buf, socket, addr, clients, entities).await;
        }
        ATTACK => {}
        HIT => {}
        DIE => {}
        _ => {} // Ignore other cases
    }
}