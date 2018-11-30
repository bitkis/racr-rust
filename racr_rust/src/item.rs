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

impl From<Use> for Item {
    fn from(item: Use) -> Item {
        Item::Use(item)
    }
}

impl From<Module> for Item {
    fn from(item: Module) -> Item {
        Item::Mod(item)
    }
}

impl From<UnitDefinition> for Item {
    fn from(item: UnitDefinition) -> Item {
        Item::Unit(item)
    }
}

impl From<PeripheralDefinition> for Item {
    fn from(item: PeripheralDefinition) -> Item {
        Item::Peripheral(item)
    }
}

impl From<RegisterDefinition> for Item {
    fn from(item: RegisterDefinition) -> Item {
        Item::Register(item)
    }
}
