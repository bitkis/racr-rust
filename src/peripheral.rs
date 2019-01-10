use std::fmt;
use crate::indent;

use crate::ident::Ident;
use crate::path::Path;

use crate::register::RegisterSlot;

/// A RACR `peripheral` definition.
#[derive(Debug, PartialEq, Clone)]
pub struct PeripheralDefinition {
    pub ident: Ident,
    pub documentation: Option<String>,

    pub registers: Vec<RegisterSlot>,
}

/// An instantiation of a peripheral inside a `device` definition.
#[derive(Debug, PartialEq, Clone)]
pub struct PeripheralInstance {
    pub ident: Ident,
    pub path: Path,
    pub address: usize,
}

impl PeripheralDefinition {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        // Print description if it exists
        if let Some(ref documentation) = self.documentation {
            writeln!(f, "{}#[doc = \"{}\"]", indent::string(indent_level), documentation)?;
        }

        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for reg in self.registers.iter() {
            writeln!(f, "{}{},", indent::string(indent_level+1), reg)?;
        }
        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "}}")
    }
}

impl fmt::Display for PeripheralDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}
