// I want to write a udp server in rust with tokio and spawn a new async task for each received message to process it concurrently. 
// The concurrent tasks need to store the information received and share the data between them.

// To create a UDP server in Rust using Tokio and spawn a new async task for each received message to process concurrently, you can follow these steps:

// Add the necessary dependencies in your Cargo.toml:

// [dependencies]
// tokio = { version = "1", features = ["full"] }

// Write the UDP server code using Tokio, spawning an async task for each received message and processing them concurrently. 
// Use an Arc<RwLock<T>> to store and share data between tasks:

use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

type SharedData = Arc<RwLock<Vec<u8>>>;

async fn process_msg(data: Vec<u8>, shared_data: SharedData) {
    // Process the received message
    println!("Received message: {:?}", data);

    // Store the data in the shared_data vector
    let mut shared_data_write = shared_data.write().await;
    shared_data_write.extend_from_slice(&data);
    drop(shared_data_write); // Release the write lock

    // Access the shared_data vector
    let shared_data_read = shared_data.read().await;
    println!("Shared data: {:?}", *shared_data_read);
}

async fn udp_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Server listening on {}", socket.local_addr()?);
    let mut buf = vec![0u8; 1024];
    let shared_data: SharedData = Arc::new(RwLock::new(Vec::new()));

    loop {
        let (size, addr) = socket.recv_from(&mut buf).await?;
        let data = buf[..size].to_vec();

        let shared_data = Arc::clone(&shared_data);
        tokio::spawn(async move {
            process_msg(data, shared_data).await;
        });
    }
}

#[tokio::main]
async fn main() {
    udp_server().await.unwrap();
}


// In this example, the SharedData type alias is used for convenience, and it represents an Arc<RwLock<Vec<u8>>>. 
// The process_msg function is an async function that processes the received message and accesses the shared data concurrently. 
// The udp_server function listens for incoming messages on a specified address and spawns a new async task to process each message concurrently. 
// The Arc<RwLock<T>> is used to enable shared mutable access to the data across the async tasks.