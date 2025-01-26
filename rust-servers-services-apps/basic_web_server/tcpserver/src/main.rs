use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // Initialize a socket server to bind to IP address 127.0.0.1 (localhost) and port 3000
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000");

    // The socket server waits (listens) for incoming connections.
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        // Echo back whatever is received to the client on the same connection.
        stream.write(&mut buffer).unwrap();
    }
}
