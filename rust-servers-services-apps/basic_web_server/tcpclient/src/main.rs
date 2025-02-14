use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write("hello world".as_bytes()).unwrap();
    // Read the bytes received from server.
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    )
}
