use std::fmt;

use crate::{
    Use,
    Module,
    UnitDefinition,
    PeripheralDefinition,
    RegisterDefinition,
};

pub enum Item {
    Use(Use),
    Mod(Module),
    Unit(UnitDefinition),
    Peripheral(PeripheralDefinition),
    Register(RegisterDefinition),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Use(x) => writeln!(f, "{}", x),
            Item::Mod(x) => writeln!(f, "{}", x),
            Item::Unit(x) => writeln!(f, "{}", x),
            Item::Peripheral(x) => writeln!(f, "{}", x),
            Item::Register(x) => writeln!(f, "{}", x),
        }
    }
}
