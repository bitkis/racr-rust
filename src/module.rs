use std::fmt;
use crate::indent;

use crate::item::Item;
use crate::ident::Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}

impl Module {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        if let Some(ref content) = self.content {
            write!(f, "{}", indent::string(indent_level))?;
            writeln!(f, "mod {} {{", self.ident)?;
            for item in content {
                item.write_indented(f, indent_level+1)?;
                writeln!(f, "")?;
            }
            write!(f, "{}", indent::string(indent_level))?;
            write!(f, "}}")
        } else {
            write!(f, "{}", indent::string(indent_level))?;
            write!(f, "mod {};", self.ident)
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}
