pub enum Method {
    Get, 
    Post, 
    Delete, 
    PUT
}
#[derive(Debug, Clone)]
pub struct MethodError;

#[derive(Debug, Clone)]
pub struct InvalidRequest;

#[derive(Debug, Clone)]
pub enum HttpRequestErrors {
    InvalidRequest(InvalidRequest), 
    MethodError(MethodError)
}

impl Method {
  pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "Get",
            Method::Post => "Post",
            Method::Delete => "Delete",
            Method::PUT => "PUT",
        }
    }
    fn from_str(s:&str) -> Result<Method, HttpRequestErrors> {
         match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "DELETE" => Ok(Method::Delete),
            "PUT" => Ok(Method::PUT),
            _ => Err(HttpRequestErrors::MethodError(MethodError)),
        }
        
    }
}


pub struct HttpRequest<'a> {
    pub path: String,
    pub method: Method,
    pub scheme:String,
    pub body:String,
    pub headers: Vec<&'a str>,
}

impl<'a> HttpRequest<'a>{ 
    pub fn new(a:&'a str) -> Result<Self,HttpRequestErrors> { 
        println!("{}",a.to_owned());
        let blocks = a.split("\r\n").collect::<Vec<&'a str>>();
        if blocks.is_empty() {
            return Err(HttpRequestErrors::InvalidRequest(InvalidRequest));
        }

        let req_params = blocks[0].split(' ').collect::<Vec<&str>>(); 
        if req_params.len() < 3 {
            return Err(HttpRequestErrors::InvalidRequest(InvalidRequest))
        }
        let method = Method::from_str(req_params[0])?;
        let path = req_params[1]; 
        let scheme = req_params[2];
        let mut headers : Option<Vec<&str>> = None;

        if blocks.len() >= 2 {
          headers = Some(blocks[1..blocks.len()-2].to_owned());
        }

       let headers = headers.unwrap_or_default();

        // Body should always come last
        let body = blocks[blocks.len() - 1];

       Ok(Self{ 
            path:path.to_owned(),
            method,
            scheme:scheme.to_owned(),
            body: body.to_owned(),
            headers,
        })

    }


    pub fn get_header(&self, s:&str) -> Option<&&str> {
        for header in &self.headers[..] {
            if header.contains(s) {
                print!("{}",header);
                return Some(header)
            }
        };
        None 
    }
} 

