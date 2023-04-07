// what if i want to handle each client request in a new async task inside the loop in the udp_server function?

// In a UDP server, there's no concept of connections like in a TCP server.
// However, you can still spawn a new async task for each received message to process it concurrently. Here's an example of how to do this using Tokio:

use tokio::net::UdpSocket;
use tokio::runtime;
use tokio::spawn;

const SERVER_ADDR: &str = "127.0.0.1:8080";

async fn handle_request(socket: &UdpSocket, data: &[u8], src: std::net::SocketAddr) {
    println!("Received from {}: {}", src, String::from_utf8_lossy(data));

    // Echo the received data back to the sender
    let sent = socket.send_to(data, &src).await.unwrap();
    println!("Sent {} bytes back to {}", sent, src);
}

async fn udp_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(SERVER_ADDR).await?;
    println!("UDP server listening on {}", SERVER_ADDR);

    let mut buf = vec![0u8; 1024];

    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        let data = buf[..size].to_vec();

        let socket_clone = socket.try_clone().unwrap();
        spawn(handle_request(&socket_clone, &data, src));
    }
}

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        if let Err(e) = udp_server().await {
            eprintln!("Error: {}", e);
        }
    });
}


// In this example, we've modified the udp_server function by adding a new handle_request function that handles each incoming request.
// Inside the loop, we clone the UdpSocket and create a new Vec<u8> for each received message. Then, we call spawn to run the handle_request function concurrently.

// Please note that cloning a UdpSocket is cheap since it only creates a new reference to the same underlying socket.
// This way, we can handle multiple clients concurrently while keeping the server responsive.