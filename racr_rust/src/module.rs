use std::fmt;

use crate::item::Item;
use crate::ident::Ident;

pub struct Module {
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref content) = self.content {
            writeln!(f, "mod {} {{", self.ident)?;
            for item in content {
                writeln!(f, "{}", item)?;
            }
            write!(f, "}}")
        } else {
            write!(f, "mod {};", self.ident)
        }
    }
}
