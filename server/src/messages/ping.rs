use tokio::net::UdpSocket;
use std::sync::Arc;
use std::net::SocketAddr;

pub(crate) async fn handle_ping(
    buf: [u8;1024], 
    msg_len: usize, 
    socket: Arc<UdpSocket>, 
    addr: SocketAddr
) {
    let _ = socket.send_to(&buf[0..msg_len], &addr).await;
}