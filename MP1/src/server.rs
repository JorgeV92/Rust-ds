use std::{
    fmt::format,  
    fs::{self, OpenOptions, File}, 
    io::{Read, Write}, 
    net::TcpStream
};
use simplelog::*;
use log::{info, warn};

// Incoming connections (server-side)
pub fn sever_machine(mut stream: TcpStream, machine_id: u32) {
    let mut buffer = [0; 1024];

    let log_file = File::create(format!("machine.{}.log", machine_id)).unwrap();
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).unwrap();

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            info!("Acknowledge of query: {}", request);

            let log_file = format!("machine.{}.log", machine_id);
            match fs::read_to_string(&log_file) {
                Ok(log_output) => {
                    let response = format!("Machine {} log: \n{}", machine_id, log_output);
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        warn!("Failed to send response: {}", e);
                    } 
                }
                Err(e) => {
                    warn!("Failed to read log file {}: {}",log_file,  e);
                }
            }
        }
        Err(e) => {
            warn!("Failed to read from stream: {}", e);
        }
    }
}

