#[cfg(test)] extern crate mockito;
extern crate reqwest;
#[macro_use] extern crate serde_derive;

use std::fmt;

use reqwest::header::{Accept, UserAgent, qitem};


pub mod pipelines;
pub mod stages;

type Result<T> = ::std::result::Result<T, Error>;


pub struct Error {}


impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gocd error")
    }
}

pub struct Gocd {
    base_url: String,
    username: String,
    password: String
}


impl Gocd {
    pub fn new(base_url: &str, username: &str, password: &str) -> Gocd {
        Gocd {
            base_url: String::from(base_url),
            username: String::from(username),
            password: String::from(password)
        }
    }

    pub fn get(&self, url: &str, accept: Option<&str>) -> Result<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", self.base_url, url);
        let username = self.username.clone();
        let password = Some(self.password.clone());
        
        let accept = match accept {
            Some(a) => a,
            None => "application/json"
        };

        let result = client.get(&url)
            .basic_auth(username, password)
            .header(UserAgent::new(format!("Gocd-rs/{} ({})", 
                                           env!("CARGO_PKG_VERSION"),
                                           env!("CARGO_PKG_HOMEPAGE"))))
            .header(Accept(vec![qitem(accept.parse().unwrap())]))
            .send();

        match result {
            Ok(mut r) => match r.text() {
                Ok(t) => Ok(t),
                Err(e) => Err(Error {})
            },
            Err(e) => Err(Error {}) // todo error reporting
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
