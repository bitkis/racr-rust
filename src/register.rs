use std::fmt;
use crate::indent;

use crate::ident::Ident;
use crate::path::Path;
use crate::access::Access;
use crate::field::FieldInstance;

/// A RACR `register` definition.
#[derive(Debug, PartialEq, Clone)]
pub struct RegisterDefinition {
    pub access: Access,
    pub ident: Ident,
    pub documentation: Option<String>,

    pub size: usize,
    pub reset_value: Option<u128>,

    pub fields: Vec<FieldInstance>,
}

/// In a `device` definition, a "slot" can be populated by a single register or a union of registers.
#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSlot {
    Single{instance: RegisterInstance, offset: usize},
    Union{alternatives: Vec<RegisterInstance>, offset: usize},
}

/// An instantiation of a `register` inside a `peripheral` definition.
#[derive(Debug, PartialEq, Clone)]
pub struct RegisterInstance {
    pub ident: Ident, 
    pub ty: RegisterType,
}

/// A register kind of type.
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

        if self.fields.is_empty() {
            write!(f, " {{}}")
        } else {
            writeln!(f, " {{")?;
            for field in self.fields.iter() {
                field.write_indented(f, indent_level+1)?;
                writeln!(f, ",")?;
            }
            write!(f, "{}}}", indent::string(indent_level))
        }
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
            RegisterSlot::Single{instance, offset} => write!(f, "{} @ {:#x}", instance, offset),
            RegisterSlot::Union{alternatives, offset} => {
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
