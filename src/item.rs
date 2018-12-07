use std::fmt;

use crate::{
    Use,
    Module,
    DeviceDefinition,
    PeripheralDefinition,
    RegisterDefinition,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Use(Use),
    Mod(Module),
    Device(DeviceDefinition),
    Peripheral(PeripheralDefinition),
    Register(RegisterDefinition),
}

impl Item {
    pub(crate) fn write_indented<'a>(&self, f: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        match self {
            Item::Use(x) => x.write_indented(f, indent_level),
            Item::Mod(x) => x.write_indented(f, indent_level),
            Item::Device(x) => x.write_indented(f, indent_level),
            Item::Peripheral(x) => x.write_indented(f, indent_level),
            Item::Register(x) => x.write_indented(f, indent_level),
        }
    }

    pub fn is_use(&self) -> bool {
        if let Item::Use(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_mod(&self) -> bool {
        if let Item::Mod(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_device(&self) -> bool {
        if let Item::Device(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_peripheral(&self) -> bool {
        if let Item::Peripheral(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_register(&self) -> bool {
        if let Item::Register(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_definition(&self) -> bool {
        match self {
            Item::Use(_) => false,
            Item::Mod(_) => false,
            Item::Device(_) => true,
            Item::Peripheral(_) => true,
            Item::Register(_) => true,
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

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}
