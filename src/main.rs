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
    let mut buffer =  vec![0;2048];
    stream.read(&mut buffer).unwrap();
    let a  = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v, 
        _ => {
            panic!("Error");
        },
    };
    println!("A");
    let request = 
        http_request::HttpRequest::new(&a).unwrap();

    println!("b", );
    let mut resp: Option<String> = None;
    
    println!("c" );
    if request.scheme.starts_with("HTTP/1.1") {
        let path_vec = request.path.split('/').collect::<Vec<&str>>();
        if request.method.as_str() == "POST" && path_vec[1] == "files" {
            let file_name = path_vec[2];
            let dir = get_env_directory();
            let content_length = request.get_header("Content-Length").unwrap_or("");
            let content_length_int: usize = content_length.to_owned().parse().unwrap();
            let bytes = request.body.as_bytes();
            let byte_slice =   &bytes[0..content_length_int];

            if !file_name.is_empty() && !content_length.is_empty() {
                if let Ok(dir) = dir { 
                    let file = File::create(dir + "/" + file_name);

                    if let  Ok(mut file) =  file {
                        match file.write_all(byte_slice) {
                            Ok(_) => {
                                println!("Success writing file");
                                resp = Some("HTTP/1.1 201 Created\r\n\r\n".to_owned());
                            },
                            Err(error) => {println!("Error: {}", error)}
                        }             
                    } 
                } 
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
                        resp = Some(http_response::get_text_plain_response(x));
                    }
                } else if path_vec[1] == "files"{
                    let file_path = path_vec.get(2);
                    if let Some(file_path) = file_path {
                        let file_resp = http_response::get_file_response(file_path);
                        if let Ok(fr) = file_resp { 
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
