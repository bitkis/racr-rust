use std::fmt;

use crate::ident::Ident;
use crate::path::Path;

use crate::register::RegisterInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct PeripheralDefinition {
    pub ident: Ident,
    pub description: Option<String>,

    pub registers: Vec<RegisterInstance>,
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
        if let Some(ref description) = self.description {
            writeln!(f, "///{}", description)?;
        }

        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for reg in self.registers.iter() {
            writeln!(f, "{}:  {} @ {:#x},", reg.ident, reg.path, reg.offset)?;
        }
        writeln!(f, "}}")
    }
}
