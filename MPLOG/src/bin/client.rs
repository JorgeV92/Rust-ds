use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use std::fs;

const HOSTS: [&str; 1] = [
    "127.0.0.1", // localhost test with my own machine 
];

const PORT: u16 = 7878; // Might need to change port but this works for now 

// Data structure for log entry (received from server) 
// for working with json 
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    log_path: String,
    host: String,
    port: String,
    line_number: usize,
    content: String,
}

// Clean up old .temp files
fn rm_temp_files() {
    let current_dir = std::env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("temp") {
            println!("[INFO] clean up old temp file for the current directory: {:?}", path);
            fs::remove_file(path).unwrap();
        }
    }
}

// Function to query a single host (Machine i)  
fn query_host(pattern: &str, host: &str, port: u16) -> Result<(String, usize, Duration), String> {
    let start = Instant::now();
    let addr = format!("{}:{}", host, port);
    let mut logs: Vec<LogEntry> = Vec::new();
    
    // Establish a connection to the server 
    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            // Send the pattern as JSON
            let request = json!({ "pattern": pattern });
            let request_str = request.to_string();
            stream.write_all(request_str.as_bytes()).unwrap();

            // Receive data from the server
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            
            if !buffer.is_empty() {
                logs = serde_json::from_slice(&buffer).unwrap();
            }

            // Write the logs to a .temp file
            let temp_file_path = format!("{}.temp", host);
            let temp_file = File::create(&temp_file_path).unwrap();
            let mut writer = BufWriter::new(temp_file);

            let mut count = 0;
            for log in logs {
                count += 1;
                writeln!(
                    writer,
                    "{} {} {} {} {}",
                    log.host, log.port, log.log_path, log.line_number, log.content
                )
                .unwrap();
            }
            writer.flush().unwrap();

            let duration = start.elapsed();
            Ok((host.to_string(), count, duration))
        }
        Err(e) => Err(format!("[ERROR] Failed to connect to {}: {}", host, e)),
    }
}

// Client that queries multiple hosts in parallel
fn query(pattern: &str) {
    // Clean any old temp files
    rm_temp_files();

    let (tx, rx) = mpsc::channel();
    let start = Instant::now();

    // Launch queries in parallel 
    for &host in HOSTS.iter() {
        let pattern = pattern.to_string();
        let tx = tx.clone();
        thread::spawn(move || {
            let result = query_host(&pattern, host, PORT);
            tx.send(result).unwrap();
        });
    }

    // Collect results from each host
    let mut total_lines = 0;
    let mut time_taken = Instant::now().duration_since(start);
    for _ in 0..HOSTS.len() {
        match rx.recv() {
            Ok(Ok((host, lines, duration))) => {
                println!(
                    "From Machine: {}, {} lines matched, took {:.4} seconds",
                    host, lines, duration.as_secs_f64()
                );
                total_lines += lines;
                if duration > time_taken {
                    time_taken = duration;
                }
            }
            Ok(Err(e)) => {
                println!("{}", e);
            }
            Err(_) => {
                println!("[ERROR] Thread panicked.");
            }
        }
    }

    println!(
        "\n🦀==========  Covfefe Software with Rust  ==========🦀\nTotal {} lines matched across all hosts, total time: {:.4} seconds",
        total_lines, time_taken.as_secs_f64()
    );    
}

fn main() {
    let pattern = std::env::args().nth(1).expect("[ERROR] No pattern provided");
    query(&pattern);
}
