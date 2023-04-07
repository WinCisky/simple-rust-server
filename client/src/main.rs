use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::interval;
use rand::prelude::*;

const PING: u8 = 0x01;
const UPDATE_POSITION: u8 = 0x02;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:8080").await?;
    let mut rng = rand::thread_rng();

    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;

        let mut buf = [0; 1024];
        buf[0] = UPDATE_POSITION;

        let pos_x: u32 = rng.gen();
        let pos_y: u32 = rng.gen();

        //random x pos
        buf[1] = (pos_x >> 24) as u8;
        buf[2] = (pos_x >> 16) as u8;
        buf[3] = (pos_x >> 8) as u8;
        buf[4] = pos_x as u8;
        //random y pos
        buf[5] = (pos_y >> 24) as u8;
        buf[6] = (pos_y >> 16) as u8;
        buf[7] = (pos_y >> 8) as u8;
        buf[8] = pos_y as u8;

        println!("Sending pos x: {}, pos y: {}", pos_x, pos_y);


        socket.send(&buf[0..10]).await?;
        let n = socket.recv(&mut buf).await?;

        println!("Received {} bytes from the server", n);

        if buf[0] == UPDATE_POSITION && n >= 9 {
            let pos_x = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
            let pos_y = u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]);

            println!("Received pos x: {}, pos y: {}", pos_x, pos_y);
        }
    }
}
