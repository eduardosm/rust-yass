// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate yass_codegen;

use yass_codegen::schema as sch;

const EXAMPLE_1_SCHEMA: sch::Schema<'static> = sch::Schema {
    header: "example-1-widget",
    root_type: "widget",
    type_defs: &[
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "widget",
            code_name: "Widget",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "debug",
                    code_name: "debug",
                    mode: sch::StructFieldMode::SingleOptional,
                    type_: sch::Type::Bool,
                },
                sch::StructFieldDef {
                    yass_name: "window",
                    code_name: "window",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Defined("window"),
                },
                sch::StructFieldDef {
                    yass_name: "item",
                    code_name: "items",
                    mode: sch::StructFieldMode::MultipleRequired,
                    type_: sch::Type::Defined("item"),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "window",
            code_name: "Window",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "title",
                    code_name: "title",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Utf8String,
                },
                sch::StructFieldDef {
                    yass_name: "name",
                    code_name: "name",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::AsciiString,
                },
                sch::StructFieldDef {
                    yass_name: "width",
                    code_name: "width",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::UInt32,
                },
                sch::StructFieldDef {
                    yass_name: "height",
                    code_name: "height",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::UInt32,
                },
            ],
        }),
        sch::TypeDef::TaggedUnion(sch::TaggedUnionDef {
            yass_name: "item",
            code_name: "Item",
            variants: &[
                sch::VariantDef {
                    yass_name: "text",
                    code_name: "Text",
                    type_: sch::Type::Defined("text"),
                },
                sch::VariantDef {
                    yass_name: "image",
                    code_name: "Image",
                    type_: sch::Type::Defined("image"),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "text",
            code_name: "Text",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "data",
                    code_name: "data",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Utf8String,
                },
                sch::StructFieldDef {
                    yass_name: "name",
                    code_name: "name",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::AsciiString,
                },
                sch::StructFieldDef {
                    yass_name: "hoffset",
                    code_name: "hoffset",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int32,
                },
                sch::StructFieldDef {
                    yass_name: "voffset",
                    code_name: "voffset",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int32,
                },
                sch::StructFieldDef {
                    yass_name: "size",
                    code_name: "size",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::UInt32,
                },
                sch::StructFieldDef {
                    yass_name: "style",
                    code_name: "style",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Defined("text-style"),
                },
                sch::StructFieldDef {
                    yass_name: "alignment",
                    code_name: "alignment",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Defined("text-alignment"),
                },
                sch::StructFieldDef {
                    yass_name: "on-mouse-up",
                    code_name: "on_mouse_up",
                    mode: sch::StructFieldMode::SingleOptional,
                    type_: sch::Type::ByteString,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "image",
            code_name: "Image",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "src",
                    code_name: "src",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::ByteString,
                },
                sch::StructFieldDef {
                    yass_name: "name",
                    code_name: "name",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::AsciiString,
                },
                sch::StructFieldDef {
                    yass_name: "hoffset",
                    code_name: "hoffset",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int32,
                },
                sch::StructFieldDef {
                    yass_name: "voffset",
                    code_name: "voffset",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int32,
                },
                sch::StructFieldDef {
                    yass_name: "alignment",
                    code_name: "alignment",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Defined("text-alignment"),
                },
                sch::StructFieldDef {
                    yass_name: "on-mouse-up",
                    code_name: "on_mouse_up",
                    mode: sch::StructFieldMode::SingleOptional,
                    type_: sch::Type::ByteString,
                },
            ],
        }),
        sch::TypeDef::Enum(sch::EnumDef {
            yass_name: "text-style",
            code_name: "TextStyle",
            values: &[
                sch::EnumValueDef {
                    yass_name: "normal",
                    code_name: "Normal",
                },
                sch::EnumValueDef {
                    yass_name: "bold",
                    code_name: "Bold",
                },
                sch::EnumValueDef {
                    yass_name: "italic",
                    code_name: "Italic",
                },
            ],
            unknown_value_name: None,
        }),
        sch::TypeDef::Enum(sch::EnumDef {
            yass_name: "text-alignment",
            code_name: "TextAlignment",
            values: &[
                sch::EnumValueDef {
                    yass_name: "left",
                    code_name: "Left",
                },
                sch::EnumValueDef {
                    yass_name: "center",
                    code_name: "Center",
                },
                sch::EnumValueDef {
                    yass_name: "right",
                    code_name: "Right",
                },
            ],
            unknown_value_name: None,
        }),
    ],
};

fn main() {
    let code = yass_codegen::gen_code_for_schema_as_string(&EXAMPLE_1_SCHEMA);
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let code_file_path = std::path::Path::new(&out_dir).join("example_1_schema.rs");
    let mut code_file = std::fs::OpenOptions::new()
        .write(true).create(true).truncate(true).open(code_file_path).unwrap();
    
    use std::io::Write;
    code_file.write_all(code.as_bytes()).unwrap();
    code_file.flush().unwrap();
}
