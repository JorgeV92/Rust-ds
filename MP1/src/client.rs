use std::{
    fs,
    io::{prelude::*, BufReader, Write, Read},
    net::{TcpStream},
};


pub fn client_machine(ip_address: &str, query: &str) -> String {
    let mut stream = TcpStream::connect(ip_address).unwrap();
    stream.write_all(query.as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..]).to_string()
}