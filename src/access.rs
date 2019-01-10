use std::fmt;

/// Specify if the associated item allows reads and/or writes.
/// 
/// The access specifier is mandatory on registers and optional on fields.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Access {
    /// Register or field is ReadOnly (`ro`)
    ReadOnly,

    /// Register or field is WriteOnly (`wo`)
    WriteOnly,

    /// Register or field is ReadWrite (`rw`) and allows both Read and Write.
    ReadWrite,

    /// Register or field is ReadAsWrite (`raw`)
    /// 
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