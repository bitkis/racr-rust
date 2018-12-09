use std::ops::Range;

use crate::ident::Ident;
use crate::access::Access;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInstance {
    pub ident: Ident,
    pub documentation: Option<String>,
    pub bit_range: Range<usize>,
    pub access: Option<Access>,
    pub variants: Vec<FieldVariant>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldVariant {
    pub ident: Ident,
    pub documentation: Option<String>,
    pub value: u128,
}
