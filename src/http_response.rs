

pub fn create_text_plain_response(body:&str) -> String {
 let content_length = body.len();
 format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
    content_length.to_string(), body)
}
