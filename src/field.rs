use crate::ident::Ident;
use crate::access::Access;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInstance {
    pub ident: Ident,
    pub documentation: Option<String>,
    pub bit_start: usize,
    pub bit_end: usize,
    pub access: Option<Access>,
    pub variants: Vec<FieldVariant>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldVariant {
    pub ident: Ident,
    pub value: u128,
}
