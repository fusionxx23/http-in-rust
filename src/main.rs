// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    str
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
//
fn handle_stream(mut stream: TcpStream) {
    let mut buffer =[0; 512];
    stream.read(&mut buffer).unwrap();
    let a  = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v, 
        _ => {
            panic!("Error");
        },
    };
    let b = a.split(' ').collect::<Vec<&str>>();
    let method = b[0]; 
    let path = b[1]; 
    let scheme = b[2];
    let resp: mut &str = "HTTP/1.1 404 Not Found\r\n\r\n"; 
    
    if method == "GET"  { 
        if scheme.starts_with("HTTP/1.1\r\n") {
            let path_vec = path.split("/").collect::<Vec<&str>>();
            if path_vec[1] == "echo" {
               
                let content_length  = path_vec[2].len().to_string();
                if(content_length > 0) { resp = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length:" 
                    + content_length + "\r\n\r\n" + path_vec[2];
                }
            }
        }
    }

    //
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap()
}
