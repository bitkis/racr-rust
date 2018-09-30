use std::fmt;

use crate::ident::Ident;
use crate::access::Access;
use crate::field::FieldInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterDefinition {
    pub access: Access,
    pub ident: Ident,
    pub description: Option<String>,

    pub size: usize,
    pub reset_value: Option<usize>,

    pub overlapping: bool,

    pub fields: Vec<FieldInstance>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterInstance {
    pub ident: Ident,
    pub reg: Ident,
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
        if self.overlapping {
            write!(f, " : overlapping")?;
        }
        writeln!(f, " {{")?;

        for field in self.fields.iter() {
            if let Some(ref description) = field.description {
                writeln!(f, "{}", description)?;
            }
            writeln!(f, "{}  {}[{}..{}],", field.access, field.ident, field.bit_start, field.bit_end)?;
        }
        writeln!(f, "}}")
    }
}