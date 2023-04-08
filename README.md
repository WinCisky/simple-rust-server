# Simple Rust Server

This project aims to preoduce a simple udp server using [rust](https://www.rust-lang.org) and [tokio](https://tokio.rs/) with reliability and performace in mind.

## Implemented Features

### ping (0x01)
sending a ping message to the server results in the server sending a ping message to the client that sent the message .

### position (0x02)
sending a position message will result in informing the other clients of the new client position and register the new client in the list of clients that will be informed when a client sends a position message.

## Build the Server
build with `cargo build --release`

## Client Examples

### C# send ping to server
``` C#
// connect
int serverPort = 8080;
int clientPort = 1234;
string serverIp = "127.0.0.1";
var udpClient = new UdpClient(clientPort);
var sendEndPoint = new IPEndPoint(IPAddress.Parse(serverIp), serverPort);
// send
byte[] bytes = new byte[1];
bytes[0] = 0x01;
udpClient.Send(bytes, bytes.Length, sendEndPoint);
```

### C# receive ping from server
``` C#
// connect
int clientPort = 1234;
var udpClient = new UdpClient(clientPort);
udpClient.BeginReceive(new System.AsyncCallback(ReceiveCallback), null);

void ReceiveCallback(System.IAsyncResult ar)
{
    // listen for next message
    udpClient.BeginReceive(new System.AsyncCallback(ReceiveCallback), null);
    var remoteEndPoint = new IPEndPoint(IPAddress.Any, 0);
    var receivedBytes = udpClient.EndReceive(ar, ref remoteEndPoint);

    if(receivedBytes.Length == 0) return;
    if (receivedBytes[0] == 0x01) {
        // ping received
    }
}
```

## Other info
when a client doesn't send position messages to the server for 5s the client will be considered disconnected and removed from the list.

sending a ping will not result in having the client added to the list or having the 5s timeout resetted.