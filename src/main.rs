// Uncomment this block to pass the first stage
mod http_request; 
mod http_response; 
use std::{
     env, fs::File, io::{BufReader, Read, Write}, net::{TcpListener, TcpStream}, str, thread
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

fn get_env_directory() -> Result<String, ()>   {
    let args: Vec<String> = env::args().collect();  
    let dir =  match args.get(2) {
        Some(i) => i, 
        None =>{ return Err(())},
    };
    Ok(dir.clone())
}
fn handle_stream(mut stream: TcpStream) {
    let resp = get_response(stream.clone());
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap()
}
fn get_response(mut stream: TcpStream) -> String {
    let mut buffer =  vec![0;2048];
    stream.read(&mut buffer).unwrap();
    let a  = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v, 
        _ => {
            panic!("Error");
        },
    };
    let request = 
        http_request::HttpRequest::new(&a).unwrap();
    let mut resp = "HTTP/1.1 404 Not Found\r\n\r\n".to_owned();
    
    if request.scheme.starts_with("HTTP/1.1") {
        let path_vec = request.path.split('/').collect::<Vec<&str>>();
        if request.method.as_str() == "POST" && path_vec[1] == "files" {
            let file_name = path_vec[2];
            let dir = match get_env_directory() {
                Ok(i) => i, 
                Err(_) => {return resp} // TODO RETURN INTERNAL SERVER ERROR
            };
            let content_length = request.get_header("Content-Length").unwrap_or("");
            let content_length_int: usize = content_length.to_owned().parse().unwrap();
            let bytes = request.body.as_bytes();
            let byte_slice = &bytes[0..content_length_int];
            if file_name.is_empty() || content_length.is_empty() {
                return resp 
            }
            
            let file = File::create(dir + "/" + file_name);
            if let  Ok(mut file) =  file {
                match file.write_all(byte_slice) {
                    Ok(_) => {
                        println!("Success writing file");
                        resp = "HTTP/1.1 201 Created\r\n\r\n".to_string();
                    },
                    Err(error) => {println!("Error: {}", error)}
                }             
            } 

        }

        if request.method.as_str() == "GET"  { 
                if path_vec[1] == "echo" { 
                    let content_length  = path_vec[2].len();
                    if content_length > 0 {
                        resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        content_length, path_vec[2]);
                    }
                } else if path_vec[1] == "user-agent"{
                    let user_agent = request.get_header("User-Agent");
                    if let Some(x) = user_agent {
                        resp = http_response::get_text_plain_response(x);
                    }
                } else if path_vec[1] == "files"{
                    let file_path = path_vec.get(2);
                    if let Some(file_path) = file_path {
                        let file_resp = http_response::get_file_response(file_path);
                        if let Ok(fr) = file_resp { 
                        resp = fr;
                        }
                    }
                } else if path_vec[1].is_empty() {
                    resp = "HTTP/1.1 200 OK\r\n\r\n".to_string();
                }
        }
    }
    resp

}

