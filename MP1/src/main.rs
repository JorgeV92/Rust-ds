mod server;
mod client;

use std::{
    fs::File,
    io::{prelude::*, BufReader, Write, Read},
    net::{TcpListener, TcpStream},
    thread,
};
fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listen on port 7878...");
    let machine_id = 1;
    // thread 
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        server::sever_machine(stream, machine_id);
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    });

    let machines = vec!["127.0.0.1:7878"]; // -> other mahcines to add if this works
    let query = "Error pattern"; // -> fix log pattern to ouput 

    // send the query to each machine N(10)
    for machine in machines {
        let response = client::client_machine(machine, query);
        println!("Acknowledge from {}: {}", machine, response);
    }
}
