use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Access::ReadOnly => write!(f, "ReadOnly"),
            Access::WriteOnly => write!(f, "WriteOnly"),
            Access::ReadWrite => write!(f, "ReadWrite"),
        }
    }
}