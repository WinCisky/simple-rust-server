use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::interval;
use rand::prelude::*;

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

        let pos_x: f32 = rng.gen_range(-10.0..10.0);
        let pos_y: f32 = rng.gen_range(-10.0..10.0);
        let level: u8 = rng.gen_range(0..2);

        //random x pos
        let bytes = pos_x.to_le_bytes();
        buf[1..5].copy_from_slice(&bytes);

        //random y pos
        let bytes = pos_y.to_le_bytes();
        buf[5..9].copy_from_slice(&bytes);

        //random level
        let bytes = level.to_le_bytes();
        buf[9..10].copy_from_slice(&bytes);

        println!("Sending pos x: {}, pos y: {}, level: {}", pos_x, pos_y, level);


        socket.send(&buf[0..10]).await?;
        let n = socket.recv(&mut buf).await?;

        println!("Received {} bytes from the server", n);

        if buf[0] == UPDATE_POSITION && n >= 13 {
            let pos_x  = f32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
            let pos_y = f32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]);
            let client_id = u32::from_le_bytes([buf[9], buf[10], buf[11], buf[12]]);

            println!("Received pos x: {}, pos y: {}, client: {}", pos_x, pos_y, client_id);
        }
    }
}
