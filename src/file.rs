use std::fmt;

use crate::item::Item;

#[derive(Debug, PartialEq, Clone)]
pub struct FileContent {
    pub content: Vec<Item>,
}

impl fmt::Display for FileContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self.content.iter() {
            writeln!(f, "{}", item)?;
        }
        writeln!(f, "") // end on newline
    }
}
