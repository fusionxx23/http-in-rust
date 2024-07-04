

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

pub struct HttpRequest {
    pub path: String,
    pub method: Method,
    pub scheme:String,
}

impl HttpRequest { 
     pub fn new(a:String) -> Result<Self,MethodError> { 
        let b = a.split(' ').collect::<Vec<&str>>();
        let method = Method::from_str(b[0])?;
        let path = b[1]; 
        let scheme = b[2];

       Ok(Self{ 
            path:path.to_owned(),
            method:method,
            scheme:scheme.to_owned(),
        })

    }
} 
