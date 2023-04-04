// How do I write a udp server in rust using tokio?
// To create a UDP server in Rust using Tokio, you can follow these steps:

// Add the necessary dependencies to your Cargo.toml file:

// [dependencies]
// tokio = { version = "1", features = ["full"] }


// Create the UDP server using tokio::net::UdpSocket. Here's a complete example:

use tokio::net::UdpSocket;
use tokio::runtime;

const SERVER_ADDR: &str = "127.0.0.1:8080";

async fn udp_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(SERVER_ADDR).await?;
    println!("UDP server listening on {}", SERVER_ADDR);

    let mut buf = vec![0u8; 1024];

    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        let data = &buf[..size];
        println!("Received from {}: {}", src, String::from_utf8_lossy(data));

        // Echo the received data back to the sender
        let sent = socket.send_to(data, &src).await?;
        println!("Sent {} bytes back to {}", sent, src);
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


// In this example, we create an asynchronous udp_server function that binds a UdpSocket to the SERVER_ADDR address.
//The server then enters an infinite loop, where it receives data from clients and echoes the received data back to the sender.
//The main function creates a Tokio runtime and runs the udp_server function using the block_on method.

// To test the server, you can use a UDP client like netcat or nc:

// echo "Hello, UDP server!" | nc -u 127.0.0.1 8080

// You should see the server receiving the message and sending it back to the client.
