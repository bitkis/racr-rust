use std::fmt;

use crate::ident::Ident;

/// A RACR path, one or more identifiers seperated by double colons (`::`).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path {
    pub segments: Vec<Ident>,
}

impl From<Ident> for Path {
    fn from(ident: Ident) -> Self {
        Path{segments: vec![ident]}
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut segments = self.segments.iter();

        if let Some(segment) = segments.next() {
            write!(f, "{}", segment)?;
        }

        for segment in segments {
            write!(f, "::{}", segment)?;
        }

        Ok(())
    }
}
