use std::fmt;

use crate::ident::Ident;
use crate::access::Access;

use crate::peripheral::PeripheralInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct UnitDefinition {
    pub ident: Ident,
    pub description: Option<String>,

    pub peripherals: Vec<PeripheralInstance>,
}


impl fmt::Display for UnitDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print description if it exists
        if let Some(ref description) = self.description {
            writeln!(f, "///{}", description)?;
        }

        write!(f, "peripheral {}", self.ident)?;
        writeln!(f, " {{")?;

        for per in self.peripherals.iter() {
            writeln!(f, "{}:  {} @ {:#x},", per.ident, per.peripheral, per.address)?;
        }
        writeln!(f, "}}")
    }
}