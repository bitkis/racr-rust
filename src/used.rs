use std::fmt;
use crate::indent;

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

impl Use {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "use ")?;
        self.tree.write_indented(f, indent_level)?;
        write!(f, ";")
    }
}

impl UseTree {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        match self {
            UseTree::Ident(x) => write!(f, "{}", x),
            UseTree::Path{path_segment, sub_tree} => {
                write!(f, "{}::", path_segment)?;
                sub_tree.write_indented(f, indent_level)
            },
        }
    }
}

impl From<Ident> for UseTree {
    fn from(i: Ident) -> UseTree {
        UseTree::Ident(i)
    }
}

impl fmt::Display for Use {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}
