use std::net::SocketAddr;
use std::time::Instant;

pub struct Client {
    pub addr: SocketAddr,
    pub pos_x: f32,
    pub pos_y: f32,
    pub level: u8,
    pub user_id: u32,
    pub last_message: Instant,
}