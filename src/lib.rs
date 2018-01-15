#[macro_use] extern crate serde_derive;


mod pipelines;


struct Gocd {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
