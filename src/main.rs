// Uncomment this block to pass the first stage
mod http_request; 
mod http_response; 
use std::{
     fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, str, thread
};

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                thread::spawn(|| handle_stream(_stream));
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
    stream.read_exact(&mut buffer).unwrap();
    let a  = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v, 
        _ => {
            panic!("Error");
        },
    };
    println!("A: {}", a);
    let request = 
        http_request::HttpRequest::new(&a).unwrap();

    let mut resp: Option<String> = None;
    
    if request.scheme.starts_with("HTTP/1.1") {

        let path_vec = request.path.split('/').collect::<Vec<&str>>();
        if request.method.as_str() == "POST" && path_vec[1] == "files" {
            let file = File::create("foo.txt");
            if let  Ok(mut file) =  file {
                let bytes = request.body.as_bytes();
                if  file.write_all(bytes).is_ok() {
                    resp = Some(format!("")) 
                };
            } 
        }
        if request.method.as_str() == "GET"  { 
                if path_vec[1] == "echo" { 
                    let content_length  = path_vec[2].len();
                    if content_length > 0 {
                        resp = Some(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            content_length, path_vec[2]));
                    }
                } else if path_vec[1] == "user-agent"{
                    let user_agent = request.get_header("User-Agent");
                    if let Some(x) = user_agent {
                        let a  = x.split(": ").collect::<Vec<&str>>();
                        if a.len() > 1 {
                        resp = Some(http_response::create_text_plain_response(a[1]));
                        }
                    }
                } else if path_vec[1] == "files"{
                let file_path = path_vec.get(2);
                if let Some(file_path) = file_path {
                    let file_resp = http_response::create_file_response(file_path);
                    if let Ok(fr) = file_resp { 
                    println!("fr:{}", fr);
                    resp = Some(fr.clone());
                    }
                }
                } else if path_vec[1].is_empty() {
                resp = Some("HTTP/1.1 200 OK\r\n\r\n".to_owned());
                }
        }
    }


    let resp = match resp {
        Some(i) => i,
        None => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    }; 
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap()
}
