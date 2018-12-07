use std::fmt;
use crate::indent;

use crate::ident::Ident;

use crate::peripheral::PeripheralInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct DeviceDefinition {
    pub ident: Ident,
    pub documentation: Option<String>,

    pub peripherals: Vec<PeripheralInstance>,
}

impl DeviceDefinition {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        // Print documentation if it exists
        if let Some(ref documentation) = self.documentation {
            writeln!(f, "{}#[doc = \"{}\"]", indent::string(indent_level), documentation)?;
        }

        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for per in self.peripherals.iter() {
            writeln!(f, "{}{}:  {} @ {:#x},", indent::string(indent_level+1), per.ident, per.path, per.address)?;
        }
        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "}}")
    }
}

impl fmt::Display for DeviceDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}
