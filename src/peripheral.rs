use std::fmt;

use crate::ident::Ident;
use crate::path::Path;

use crate::register::RegisterSlot;

#[derive(Debug, PartialEq, Clone)]
pub struct PeripheralDefinition {
    pub ident: Ident,
    pub documentation: Option<String>,

    pub registers: Vec<RegisterSlot>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PeripheralInstance {
    pub ident: Ident,
    pub path: Path,
    pub address: usize,
}

impl fmt::Display for PeripheralDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print description if it exists
        if let Some(ref documentation) = self.documentation {
            writeln!(f, "#[doc = \"{}\"]", documentation)?;
        }

        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for reg in self.registers.iter() {
            writeln!(f, "{},", reg)?;
        }
        writeln!(f, "}}")
    }
}
