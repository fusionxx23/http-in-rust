use std::{env, fs};


pub fn get_text_plain_response(body:&str) -> String {
 let content_length = body.len();
 format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
    content_length, body)
}

fn read_file(path:&str) ->  Result<String, FileRespError> {

    let contents = fs::read_to_string(path); 
    let content = match contents {
        Ok(i) => i,
        Err(err) => {
            return Err(FileRespError::Io(err))
        },
    };
    Ok(content)
}

pub enum FileRespError {
    Io(std::io::Error), 
    ArgumentNotFound(String),
}

pub fn get_file_response(path:&str) -> Result<String, FileRespError> {

    let args: Vec<String> = env::args().collect();  
    let dir = args.get(2);

    if let Some(dir) = dir {
        let file_path = dir.to_owned() + "/" + path;
        let content = read_file(&file_path);
        let content = match content {
            Ok(i) => i, 
            Err(error) => {return Err(error)}
        };
        let content_length = content.len();

        Ok(format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
        content_length, content))
    } else {
        Err(FileRespError::ArgumentNotFound("No directory argument found.".to_owned()))
    }
}
