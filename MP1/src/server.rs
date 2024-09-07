use std::{
    fs, 
    io::{Write, Read},
    net::{TcpStream},
};

pub fn sever_machine(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Acknowledge of query: {}", request);


    let log_file = "sample_log.txt";
    let log_output = fs::read_to_string(log_file).unwrap();

    let response = format!("Machine log \n{}", log_output);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

