use racr::*;



#[test]
fn display_register() {
    let reg = RegisterDefinition {
        access: Access::ReadWrite,
        ident: Ident::from("RegisterName"),
        documentation: Some(String::from(" documentation")),

        size: 32,
        reset_value: Some(0),

        fields: vec![
            FieldInstance{ident: Ident::from("field0"), documentation: None, bit_range: 0..8, variants: Vec::new(), access: Some(Access::ReadWrite)},
            FieldInstance{ident: Ident::from("field1"), documentation: None, bit_range: 8..16, variants: Vec::new(), access: Some(Access::ReadOnly)},
            FieldInstance{ident: Ident::from("field2"), documentation: None, bit_range: 16..24, variants: Vec::new(), access: Some(Access::WriteOnly)},
            FieldInstance{ident: Ident::from("field3"), documentation: None, bit_range: 24..29, variants: Vec::new(), access: None},
            FieldInstance{ident: Ident::from("field4"), documentation: None, bit_range: 29..30, variants: Vec::new(), access: None},
            FieldInstance{ident: Ident::from("field5"), documentation: None, bit_range: 30..32, access: None, variants: vec![
                FieldVariant{ident: Ident::from("VARIANT0"), value: 0, documentation: Some(String::from("test doc"))},
                FieldVariant{ident: Ident::from("VARIANT1"), value: 1, documentation: None},
                FieldVariant{ident: Ident::from("VARIANT2"), value: 2, documentation: Some(String::from("test doc2"))},
            ]},
        ],
    };

    let display_string = format!("{}", reg);
    let desired_string = String::from(
"#[doc = \" documentation\"]
ReadWrite register[32] RegisterName = 0x0 {
    ReadWrite field0[0..8],
    ReadOnly field1[8..16],
    WriteOnly field2[16..24],
    field3[24..29],
    field4[29],
    field5[30..32] {
        #[doc = \"test doc\"]
        VARIANT0 = 0x0,
        VARIANT1 = 0x1,
        #[doc = \"test doc2\"]
        VARIANT2 = 0x2,
    },
}"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_register_empty() {
    let reg = RegisterDefinition {
        access: Access::ReadWrite,
        ident: Ident::from("RegisterName"),
        documentation: Some(String::from(" documentation")),

        size: 32,
        reset_value: Some(0),

        fields: Vec::new()
    };

    let display_string = format!("{}", reg);
    let desired_string = String::from(
"#[doc = \" documentation\"]
ReadWrite register[32] RegisterName = 0x0 {}"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_peripheral() {
    let reg = PeripheralDefinition {
        ident: Ident::from("PeripheralName"),
        documentation: Some(String::from(" documentation")),

        registers: vec![
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg0"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x0},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg1"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x4},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg2"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0x8},
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg3"), ty: RegisterType::Single{path: Ident::from("RegisterName").into()}}, offset: 0xc},
            RegisterSlot::Overloaded{alternatives: vec![
                    RegisterInstance{ident: Ident::from("reg4a"), ty: RegisterType::Single{path: Ident::from("A").into()}},
                    RegisterInstance{ident: Ident::from("reg4b"), ty: RegisterType::Single{path: Ident::from("B").into()}},
                    RegisterInstance{ident: Ident::from("reg4c"), ty: RegisterType::Single{path: Ident::from("C").into()}},
                ], offset: 0x10
            },
            RegisterSlot::Single{instance: RegisterInstance{ident: Ident::from("reg5"), ty: RegisterType::Array{path: Ident::from("RegisterName").into(), size: 3}}, offset: 0x14},
        ],
    };

    let display_string = format!("{}", reg);
    let desired_string = String::from(
"#[doc = \" documentation\"]
peripheral PeripheralName {
    reg0: RegisterName @ 0x0,
    reg1: RegisterName @ 0x4,
    reg2: RegisterName @ 0x8,
    reg3: RegisterName @ 0xc,
    ( reg4a: A | reg4b: B | reg4c: C ) @ 0x10,
    reg5: [RegisterName; 3] @ 0x14,
}"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_device() {
    let device = DeviceDefinition {
        ident: Ident::from("UnitName"),
        documentation: Some(String::from(" documentation")),

        peripherals: vec![
            PeripheralInstance{ident: Ident::from("peripheral0"), path: Ident::from("PeripheralName").into(), address: 0x4000_0000,},
            PeripheralInstance{ident: Ident::from("peripheral1"), path: Ident::from("PeripheralName").into(), address: 0x4000_2000},
            PeripheralInstance{ident: Ident::from("peripheral2"), path: Ident::from("PeripheralName").into(), address: 0x4000_4000},
            PeripheralInstance{ident: Ident::from("peripheral3"), path: Ident::from("PeripheralName").into(), address: 0x4000_8000},
        ],
    };

    let display_string = format!("{}", device);
    let desired_string = String::from(
"#[doc = \" documentation\"]
device UnitName {
    peripheral0: PeripheralName @ 0x40000000,
    peripheral1: PeripheralName @ 0x40002000,
    peripheral2: PeripheralName @ 0x40004000,
    peripheral3: PeripheralName @ 0x40008000,
}"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_used() {

    let used = Use {
        tree: UseTree::Path {
            path_segment: "foo".into(),
            sub_tree: Box::new(
                UseTree::Path {
                    path_segment: "bar".into(),
                    sub_tree: Box::new( UseTree::Ident("Baz".into()) )
                }
            )
        }
    };
        
    let display_string = format!("{}", used);
    let desired_string = String::from("use foo::bar::Baz;");

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_used_rename() {

    let used = Use {
        tree: UseTree::Path {
            path_segment: "foo".into(),
            sub_tree: Box::new(
                UseTree::Rename {
                    ident: "Bar".into(),
                    rename: "Baz".into(),
                }
            )
        }
    };
        
    let display_string = format!("{}", used);
    let desired_string = String::from("use foo::Bar as Baz;");

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_mod_without_content() {
    let module = Module {
        ident: "foo".into(),
        content: None,
    };
        
    let display_string = format!("{}", module);
    let desired_string = String::from("mod foo;");

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_mod_with_content() {
    let module = Module {
        ident: "foo".into(),
        content: Some(vec![
            Item::Use(Use{tree: UseTree::Ident("bar".into())}),
            Item::Use(Use{tree: UseTree::Ident("baz".into())}),
        ]),
    };
        
    let display_string = format!("{}", module);
    let desired_string = String::from(
"mod foo {
    use bar;
    use baz;
}"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_file() {
    let file_content = FileContent {
        content: vec![
            Item::Use(Use{tree: UseTree::Ident("foo".into())}),
            Item::Use(Use{tree: UseTree::Ident("bar".into())}),
            Item::Use(Use{tree: UseTree::Ident("baz".into())}),
        ],
    };
        
    let display_string = format!("{}", file_content);
    let desired_string = String::from(
"use foo;
use bar;
use baz;
"
    );

    assert_eq!(
        display_string,
        desired_string
    )
}

#[test]
fn display_path() {
    let path = Path {
        segments: vec![
            Ident::from("foo"),
            Ident::from("bar"),
            Ident::from("baz"),
        ],
    };

    let display_string = format!("{}", path);
    let desired_string = String::from("foo::bar::baz");

    // Compare
    assert_eq!(
        display_string,
        desired_string
    )
}
