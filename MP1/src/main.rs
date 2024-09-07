mod server;
mod client;

use std::{
    fs,
    io::{prelude::*, BufReader, Write, Read},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listen on port 7878...");

    // thread 
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
          server::sever_machine(stream);  
        });
    }

    let machines = vec!["127.0.0.1:7878"]; // -> other mahcines to add if this works
    let query = "Error pattern"; // -> fix log pattern to ouput 

    // send the query to each machine N(10)
    for machine in machines {
        let response = client::client_machine(machine, query);
        println!("Acknowledge from {}: {}", machine, response);
    }
}
