use std::fmt;

/// A valid RACR identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct Ident(String);

// TODO: Change to TryFrom and validate invariants
impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Ident(String::from(s))
    }
}

// TODO: Change to TryFrom and validate invariants
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