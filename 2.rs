// How do i get the remaining bytes of the request knowing the byte 1 to 5 represents an integer? 
// Assuming the message is structured such that the first byte represents the message type, 
// and the next four bytes (bytes 1 to 4) represent an integer (for example, a length of the payload), you can extract the remaining bytes of the request as follows:

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::mem::size_of;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(received_bytes) => {
            if received_bytes > 0 {
                let first_byte = buffer[0];
                let payload_len = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]) as usize;

                if payload_len > 0 && payload_len + size_of::<u32>() + 1 <= received_bytes {
                    let payload = &buffer[5..5 + payload_len];

                    match first_byte {
                        0x01 => {
                            println!("Handling message with byte 0x01");
                            // Handle message type 0x01 using payload
                        }
                        0x02 => {
                            println!("Handling message with byte 0x02");
                            // Handle message type 0x02 using payload
                        }
                        _ => {
                            println!("Unknown message type: 0x{:02X}", first_byte);
                            // Handle unknown message type
                        }
                    }
                }

                // Send a response
                let response = b"Message received";
                stream.write(response).unwrap();
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}


// In this example, we first read the message type from the first byte. We then read the next four bytes (bytes 1 to 4) as a big-endian u32 integer, which represents the length of the payload. 
// Then we extract the remaining bytes of the request as the payload. We then handle the message based on the message type and the payload.