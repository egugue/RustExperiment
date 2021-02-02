use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Started");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let contents = fs::read_to_string("../officialbook/hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
