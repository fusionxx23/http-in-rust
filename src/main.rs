#![allow(clippy::unused_io_amount)]
mod http_request;
mod http_response;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

use http_request::HttpRequest;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                std::thread::spawn(|| handle_stream(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
//

fn get_env_directory() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    let dir = match args.get(2) {
        Some(i) => i,
        None => return Err(()),
    };
    Ok(dir.clone())
}
fn handle_stream(mut stream: TcpStream) {
    let mut buffer = vec![0; 2048];
    stream.read(&mut buffer).unwrap();
    let a = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v,
        _ => {
            panic!("Error");
        }
    };
    let request = http_request::HttpRequest::new(&a).unwrap();
    let resp = get_response(request);
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn get_response(request: HttpRequest) -> String {
    let mut http_response = http_response::HttpResponse {
        status: "404 Not Found".to_string(),
        scheme: "HTTP/1.1".to_string(),
        body: "".to_string(),
        headers: vec![],
    };

    let accept_encoding = request.get_header("Accept-Encoding");

    if let Some(enc) = accept_encoding {
        if enc == "gzip" {
            http_response
                .headers
                .push("Content-Encoding: ".to_string() + enc);
        }
    }

    if request.scheme.starts_with("HTTP/1.1") {
        let path_vec = request.path.split('/').collect::<Vec<&str>>();
        if request.method.as_str() == "POST" && path_vec[1] == "files" {
            let file_name = path_vec[2];
            let dir = match get_env_directory() {
                Ok(i) => i,
                Err(_) => return http_response.get_response(), // TODO RETURN INTERNAL SERVER ERROR
            };
            let content_length = request.get_header("Content-Length").unwrap_or("");
            let content_length_int: usize = content_length.to_owned().parse().unwrap();
            let bytes = request.body.as_bytes();
            let byte_slice = &bytes[0..content_length_int];
            if file_name.is_empty() || content_length.is_empty() {
                return http_response.get_response();
            }
            let file = File::create(dir + "/" + file_name);
            if let Ok(mut file) = file {
                match file.write_all(byte_slice) {
                    Ok(_) => {
                        http_response.status = "201 Created".to_string();
                    }
                    Err(error) => {
                        println!("Error: {}", error)
                    }
                }
            }
        }

        if request.method.as_str() == "GET" {
            if path_vec[1] == "echo" {
                let nested_path = path_vec[2];
                if !nested_path.is_empty() {
                    http_response.status = "200 OK".to_string();
                    http_response
                        .headers
                        .push("Content-Type: text/plain".to_string());
                    http_response
                        .headers
                        .push(format!("Content-Length: {}", nested_path.len()));
                    http_response.body = nested_path.to_string();
                }
            } else if path_vec[1] == "user-agent" {
                let user_agent = request.get_header("User-Agent");
                if let Some(x) = user_agent {
                    http_response.status = "200 OK".to_string();
                    http_response
                        .headers
                        .push("Content-Length: ".to_string() + &x.len().to_string());
                    http_response
                        .headers
                        .push("Content-Type: text/plain".to_string());
                    http_response.body = x.to_string();
                }
            } else if path_vec[1] == "files" {
                let file_path = path_vec.get(2);
                if let Some(file_path) = file_path {
                    let file_resp = http_response::get_file_response(file_path);
                    if let Ok(fr) = file_resp {
                        http_response.status = "200 OK".to_string();
                        http_response
                            .headers
                            .push("Content-Length: ".to_string() + &fr.0.to_string());
                        http_response
                            .headers
                            .push("Content-Type: application/octet-stream".to_string());
                        http_response.body = fr.1
                    }
                }
            } else if path_vec[1].is_empty() {
                http_response.status = "200 OK".to_string();
            }
        }
    }
    http_response.get_response()
}
