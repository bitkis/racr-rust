use std::ops::Range;
use std::fmt;
use crate::indent;

use crate::ident::Ident;
use crate::access::Access;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInstance {
    pub documentation: Option<String>,
    pub bit_range: Range<usize>,
    pub access: Option<Access>,
    pub ty: FieldType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldType {
    Field{ident: Ident},
    Reserved{value: u128},
    Enum{ident: Ident, variants: Vec<FieldVariant>},
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldVariant {
    pub ident: Ident,
    pub documentation: Option<String>,
    pub value: u128,
}

impl FieldVariant {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        if let Some(ref doc) = self.documentation {
            writeln!(f, "{}#[doc = \"{}\"]", indent::string(indent_level), doc)?;
        }
        writeln!(f, "{}{} = {:#x},", indent::string(indent_level), self.ident, self.value)
    }
}

impl FieldInstance {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        if let Some(ref doc) = self.documentation {
            writeln!(f, "{}#[doc = \"{}\"]", indent::string(indent_level), doc)?;
        }

        write!(f, "{}", indent::string(indent_level))?;

        if let Some(ref access) = self.access {
            write!(f, "{} ", access)?;
        }

        match self.ty {
            FieldType::Field{..} => write!(f, "field")?,
            FieldType::Reserved{..} => write!(f, "reserved")?,
            FieldType::Enum{..} => write!(f, "enum")?,
        }

        if self.bit_range.end == self.bit_range.start + 1 {
            write!(f, "[{}]", self.bit_range.start)?;
        } else {
            write!(f, "[{}..{}]", self.bit_range.start, self.bit_range.end)?;
        }

        match self.ty {
            FieldType::Field{ref ident} => write!(f, " {}", ident),
            FieldType::Reserved{ref value} => write!(f, " = {:#x}", value),
            FieldType::Enum{ref ident, ref variants} => {
                writeln!(f, " {} {{", ident)?;
                for variant in variants {
                    variant.write_indented(f, indent_level+1)?;
                }
                write!(f, "{}", indent::string(indent_level))?;
                write!(f, "}}")
            },
        }
    }
}
