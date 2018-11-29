use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(String);

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Ident(String::from(s))
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Ident(s)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}