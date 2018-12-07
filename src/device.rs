use std::fmt;

use crate::ident::Ident;

use crate::peripheral::PeripheralInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct DeviceDefinition {
    pub ident: Ident,
    pub documentation: Option<String>,

    pub peripherals: Vec<PeripheralInstance>,
}


impl fmt::Display for DeviceDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print documentation if it exists
        if let Some(ref documentation) = self.documentation {
            writeln!(f, "#[doc = \"{}\"]", documentation)?;
        }

        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for per in self.peripherals.iter() {
            writeln!(f, "{}:  {} @ {:#x},", per.ident, per.path, per.address)?;
        }
        writeln!(f, "}}")
    }
}