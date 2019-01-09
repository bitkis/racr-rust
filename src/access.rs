use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,

    /// Same as `ReadWrite` with the additional guarantee that nothing other
    /// than register writes will change the value that is read back.
    ReadAsWrite,
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Access::ReadOnly => write!(f, "ro"),
            Access::WriteOnly => write!(f, "wo"),
            Access::ReadWrite => write!(f, "rw"),
            Access::ReadAsWrite => write!(f, "raw"),
        }
    }
}