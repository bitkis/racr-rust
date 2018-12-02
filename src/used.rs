use std::fmt;

use crate::ident::Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Use {
    pub tree: UseTree,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseTree {
    Name(UseName),
    Path(UsePath),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseName {
    pub ident: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UsePath {
    pub ident: Ident,
    pub tree: Box<UseTree>,
}

impl From<UseName> for UseTree {
    fn from(un: UseName) -> UseTree {
        UseTree::Name(un)
    }
}

impl From<UsePath> for UseTree {
    fn from(up: UsePath) -> UseTree {
        UseTree::Path(up)
    }
}

impl fmt::Display for Use {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "use {};", self.tree)
    }
}

impl fmt::Display for UseTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UseTree::Name(x) => write!(f, "{}", x),
            UseTree::Path(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for UseName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl fmt::Display for UsePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.ident, self.tree)
    }
}
