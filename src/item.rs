use std::fmt;

use crate::{
    Use,
    Module,
    DeviceDefinition,
    PeripheralDefinition,
    RegisterDefinition,
};

pub enum Item {
    Use(Use),
    Mod(Module),
    Device(DeviceDefinition),
    Peripheral(PeripheralDefinition),
    Register(RegisterDefinition),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Use(x) => write!(f, "{}", x),
            Item::Mod(x) => write!(f, "{}", x),
            Item::Device(x) => write!(f, "{}", x),
            Item::Peripheral(x) => write!(f, "{}", x),
            Item::Register(x) => write!(f, "{}", x),
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

impl From<DeviceDefinition> for Item {
    fn from(item: DeviceDefinition) -> Item {
        Item::Device(item)
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
