use std::fs::File;
use std::io::{BufRead, BufReader, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::path::PathBuf;

// Data structure for storing the matched log results
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    log_path: String,
    host: String,
    port: String,
    line_number: usize,
    content: String,
}

fn get_log_path() -> Option<PathBuf> {
    // Get the current directory (or any desired root path)
    let root = env::current_dir().unwrap();
    // Search for a file that ends with ".log" 
    for entry in root.read_dir().expect("Failed to read directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("log") {
            return Some(path);
        }
    }
    None
}

fn handle_client(mut stream: TcpStream, log_path: PathBuf, host: &str, port: u16) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            // Decode the JSON from the client
            let data = String::from_utf8_lossy(&buffer[..size]);
            let query: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON received");
            let pattern = query["pattern"].as_str().expect("No pattern provided");

            let mut line_number = 0;
            let mut matches: Vec<LogEntry> = Vec::new();

            // Open the log file and search for the pattern using regex
            let file = File::open(log_path.clone()).expect("Failed to open log file");
            let reader = BufReader::new(file);
            let re = Regex::new(pattern).expect("Invalid regex pattern");

            for line in reader.lines() {
                let line = line.unwrap();
                line_number += 1;

                if re.is_match(&line) {
                    // Add matched result to the buffer
                    matches.push(LogEntry {
                        log_path: log_path.to_string_lossy().to_string(),
                        host: host.to_string(),
                        port: port.to_string(),
                        line_number,
                        content: line,
                    });
                }
            }

            //  JSON and send them to the client
            if !matches.is_empty() {
                let response = serde_json::to_string(&matches).expect("Failed to serialize log entries");
                stream.write_all(response.as_bytes()).expect("Failed to send response");
            }
        }
        Err(e) => {
            println!("[ERROR]: Failed to read from connection: {}", e);
        }
        _ => {}
    }
}

fn main() {
    let host = "0.0.0.0";
    let port = 7878;

    let listener = TcpListener::bind((host, port)).expect("Failed to bind to port");
    println!("[INFO]: Server listening on {}:{}", host, port);

    let log_path = get_log_path().expect("No log file found in the directory");

    // Accept incoming connections and spawn a thread to handle each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let log_path = log_path.clone(); // Clone the log file path for each thread
                thread::spawn(move || {
                    handle_client(stream, log_path, host, port);
                });
            }
            Err(e) => {
                println!("[ERROR]: Connection failed: {}", e);
            }
        }
    }
}
