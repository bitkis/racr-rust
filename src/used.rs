use std::fmt;

use crate::ident::Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Use {
    pub tree: UseTree,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseTree {
    Ident(Ident),
    Path{path_segment: Ident, sub_tree: Box<UseTree>},
}

impl From<Ident> for UseTree {
    fn from(i: Ident) -> UseTree {
        UseTree::Ident(i)
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
            UseTree::Ident(x) => write!(f, "{}", x),
            UseTree::Path{path_segment, sub_tree} => write!(f, "{}::{}", path_segment, sub_tree),
        }
    }
}
