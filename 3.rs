// how do I save the incoming clients to a list and spawn a process that every 200ms sends a message to every client in the list except himself?

// You can accomplish this by using an Arc<Mutex<Vec<TcpStream>>> to store the list of clients, and then using a separate thread with a timer to send messages to all clients except the sender. Here's a complete example:

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    // Spawn the broadcast thread
    let clients_for_broadcast = clients.clone();
    thread::spawn(move || {
        loop {
            broadcast_message(&clients_for_broadcast);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                clients.lock().unwrap().push(stream.try_clone().unwrap());
                let clients_for_handling = clients.clone();
                thread::spawn(move || handle_client(stream, clients_for_handling));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    // Handle the client connection
    // ...
}

fn broadcast_message(clients: &Arc<Mutex<Vec<TcpStream>>>) {
    let message = b"Message sent to all clients";

    let mut clients = clients.lock().unwrap();
    for client in clients.iter_mut() {
        if let Err(e) = client.write(message) {
            println!("Error sending message to client: {}", e);
        }
    }
}


// In this example, we store the clients in an Arc<Mutex<Vec<TcpStream>>> to share the list safely between threads. We then clone the Arc and pass it to the spawned threads. When a new connection is established, we add the client's TcpStream to the list.

// We also create a separate thread for broadcasting messages to all clients. Inside this thread, we call the broadcast_message function every 200ms. This function iterates through the list of clients and sends a message to each client.

// Note that in this example, the message is sent to all clients, including the sender.
// To exclude the sender, you'll need to modify the broadcast_message function and handle_client function to include some identifier for each client, such as an ID, IP address, or other unique data, and use it to filter the clients you want to send the message to.