use std::{
    fmt::format, fs, io::{prelude::*, BufReader, Read, Write}, net::TcpStream
};

// client to send log query to a specific machine (client-side)
pub fn client_machine(ip_address: &str, query: &str) -> String {
    match TcpStream::connect(ip_address) {
        Ok(mut stream) => {
            stream.write_all(query.as_bytes()).unwrap();

            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => String::from_utf8_lossy(&buffer[..]).to_string(),
                Err(e) => format!("Failed to read response {}", e),
            }
        }
        Err(e) => format!("Failed ot connect to {}: {}", ip_address, e),
    }

}