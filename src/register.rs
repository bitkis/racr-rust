use std::fmt;
use crate::indent;

use crate::ident::Ident;
use crate::path::Path;
use crate::access::Access;
use crate::field::FieldInstance;

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterDefinition {
    pub access: Access,
    pub ident: Ident,
    pub documentation: Option<String>,

    pub size: usize,
    pub reset_value: Option<u128>,

    pub fields: Vec<FieldInstance>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSlot {
    Single{instance: RegisterInstance, offset: usize},
    Overloaded{alternatives: Vec<RegisterInstance>, offset: usize},
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterInstance {
    pub ident: Ident, 
    pub ty: RegisterType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterType {
    Single{path: Path},
    Array{path: Path, size: usize},
}

impl RegisterDefinition {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        // Print description if it exists
        if let Some(ref documentation) = self.documentation {
            writeln!(f, "{}#[doc = \"{}\"]", indent::string(indent_level), documentation)?;
        }

        write!(f, "{}", indent::string(indent_level))?;
        write!(f, "{} register[{}] {}", self.access, self.size, self.ident)?;
        if let Some(reset_value) = self.reset_value {
            write!(f, " = {:#x}", reset_value)?;
        }

        writeln!(f, " {{")?;
        for field in self.fields.iter() {
            if let Some(ref documentation) = field.documentation {
                write!(f, "{}", indent::string(indent_level+1))?;
                writeln!(f, "#[doc = \"{}\"]", documentation)?;
            }
            write!(f, "{}", indent::string(indent_level+1))?;
            if let Some(ref access) = field.access {
                write!(f, "{} ", access)?;
            }
            writeln!(f, "{}[{}..{}],", field.ident, field.bit_start, field.bit_end)?;
        }
        write!(f, "{}}}", indent::string(indent_level))
    }
}

impl fmt::Display for RegisterDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl fmt::Display for RegisterSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterSlot::Single{instance, offset} => write!(f, "{} @ {:#x} ", instance, offset),
            RegisterSlot::Overloaded{alternatives, offset} => {
                write!(f, "( ")?;
                for (i, alternative) in alternatives.iter().enumerate() {
                    if i != 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", alternative)?;
                }
                write!(f, " ) @ {:#x}", offset)
            },
        }
    }
}

impl fmt::Display for RegisterInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

impl fmt::Display for RegisterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterType::Single{path} => write!(f, "{}", path),
            RegisterType::Array{path, size} => write!(f, "[{}; {}]", path, size),
        }
    }
}
