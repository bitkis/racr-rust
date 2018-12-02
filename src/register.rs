use std::fmt;

use crate::ident::Ident;
use crate::path::Path;
use crate::access::Access;
use crate::field::FieldInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterDefinition {
    pub access: Access,
    pub ident: Ident,
    pub description: Option<String>,

    pub size: usize,
    pub reset_value: Option<u128>,

    pub fields: Vec<FieldInstance>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterInstance {
    pub ident: Ident,
    pub path: Path,
    pub offset: usize,
}

impl fmt::Display for RegisterDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print description if it exists
        if let Some(ref description) = self.description {
            writeln!(f, "///{}", description)?;
        }

        write!(f, "{} register[{}] {}", self.access, self.size, self.ident)?;
        if let Some(reset_value) = self.reset_value {
            write!(f, " = {:#x}", reset_value)?;
        }

        writeln!(f, " {{")?;
        for field in self.fields.iter() {
            if let Some(ref description) = field.description {
                writeln!(f, "///{}", description)?;
            }
            if let Some(ref access) = field.access {
                write!(f, "{} ", access)?;
            }
            writeln!(f, "{}[{}..{}],", field.ident, field.bit_start, field.bit_end)?;
        }
        writeln!(f, "}}")
    }
}