

pub enum Method {
    GET, 
    POST, 
    DELETE, 
    PUT
}
#[derive(Debug, Clone)]
pub struct MethodError;

impl Method {
  pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::DELETE => "DELETE",
            Method::PUT => "PUT",
        }
    }
    fn from_str(s:&str) -> Result<Method, MethodError> {
        let a =  match s.as_ref() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "DELETE" => Ok(Method::DELETE),
            "PUT" => Ok(Method::PUT),
            _ => Err(MethodError),
        }; 
        a
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
    pub fn new(a:&'a str) -> Result<Self,MethodError> { 
        println!("{}",a.to_owned());
        let blocks = a.split("\r\n").collect::<Vec<&'a str>>();
        if blocks.len() < 1 {
            return Err(MethodError);
        }

        let req_params = blocks[0].split(' ').collect::<Vec<&str>>(); 
        if req_params.len() < 3 {
            return Err(MethodError)
        }
        let method = Method::from_str(req_params[0])?;
        let path = req_params[1]; 
        let scheme = req_params[2];
        let mut headers : Option<Vec<&str>> = None;

        if blocks.len() >= 2 {
          headers = Some(blocks[1..blocks.len()-2].to_owned());
        }

       let headers = match headers {
            Some(i) => i, 
            None => vec![]
        };

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
                print!("{}",header.to_owned());
                return Some(header)
            }
        };
        None 
    }
} 

