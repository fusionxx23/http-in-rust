use std::{env, fmt::Display, fs, usize};

use crate::http_request::{HttpRequest, Method};

pub fn get_text_plain_response(body: &str) -> String {
    let content_length = body.len();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content_length, body
    )
}

fn read_file(path: &str) -> Result<String, FileRespError> {
    let contents = fs::read_to_string(path);
    let content = match contents {
        Ok(i) => i,
        Err(err) => return Err(FileRespError::Io(err)),
    };
    Ok(content)
}

pub struct HttpResponse {
    pub status: String,
    pub scheme: String,
    pub body: String,
    pub headers: Vec<String>,
}
#[derive(Debug)]
pub enum FileRespError {
    Io(std::io::Error),
    ArgumentNotFound(String),
}

impl Display for FileRespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ArgumentNotFound(_) => "No directory argument found.",
            Self::Io(_) => "IO Error.",
        };
        write!(f, "Error: {}", message)
    }
}

impl HttpResponse {
    pub fn get_response(&self) -> String {
        let headers = &self.headers.join("\r\n");
        format!(
            "{} {}\r\n{}\r\n\r\n{}",
            &self.scheme, &self.status, headers, &self.body
        )
    }
}

pub fn get_encoding(request: &HttpRequest) -> Option<String> {
    let req_encoding = match request.get_header("Accept-Encoding") {
        Some(i) => i,
        None => return None,
    };
    if req_encoding.eq("gzip") {
        return Some("Content-Encoding: gzip".to_string());
    }
    None
}

pub fn get_file_response(path: &str) -> Result<(usize, String), FileRespError> {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(2);

    if let Some(dir) = dir {
        let file_path = dir.to_owned() + "/" + path;
        let content = read_file(&file_path);
        let content = match content {
            Ok(i) => i,
            Err(error) => return Err(error),
        };
        let content_length = content.len();

        Ok((content_length, content))
    } else {
        Err(FileRespError::ArgumentNotFound(
            "No directory argument found.".to_owned(),
        ))
    }
}
