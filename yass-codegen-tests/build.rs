// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate yass_codegen;

use yass_codegen::schema as sch;

const TEST_1_SCHEMA: sch::Schema<'static> = sch::Schema {
    header: "test",
    root_type: "empty-struct",
    type_defs: &[
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "empty-struct",
            code_name: "EmptyStruct",
            fields: &[],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "struct-with-single-required",
            code_name: "StructWithSingleRequired",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "field",
                    code_name: "field",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::RawAtom,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "struct-with-single-optional",
            code_name: "StructWithSingleOptional",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "field",
                    code_name: "field",
                    mode: sch::StructFieldMode::SingleOptional,
                    type_: sch::Type::RawAtom,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "struct-with-multiple-required",
            code_name: "StructWithMultipleRequired",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "field",
                    code_name: "field",
                    mode: sch::StructFieldMode::MultipleRequired,
                    type_: sch::Type::RawAtom,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "struct-with-multiple-optional",
            code_name: "StructWithMultipleOptional",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "field",
                    code_name: "field",
                    mode: sch::StructFieldMode::MultipleOptional,
                    type_: sch::Type::RawAtom,
                },
            ],
        }),
        sch::TypeDef::TaggedUnion(sch::TaggedUnionDef {
            yass_name: "tagged-union-1",
            code_name: "TaggedUnion1",
            variants: &[
                sch::VariantDef {
                    yass_name: "variant-1",
                    code_name: "Variant1",
                    type_: sch::Type::Int32,
                },
                sch::VariantDef {
                    yass_name: "variant-2",
                    code_name: "Variant2",
                    type_: sch::Type::Bool,
                },
            ],
        }),
        sch::TypeDef::Enum(sch::EnumDef {
            yass_name: "enum-without-unknown",
            code_name: "EnumWithoutUnknown",
            values: &[
                sch::EnumValueDef {
                    yass_name: "value-1",
                    code_name: "Value1",
                },
                sch::EnumValueDef {
                    yass_name: "value-2",
                    code_name: "Value2",
                },
            ],
            unknown_value_name: None,
        }),
        sch::TypeDef::Enum(sch::EnumDef {
            yass_name: "enum-with-unknown",
            code_name: "EnumWithUnknown",
            values: &[
                sch::EnumValueDef {
                    yass_name: "value-1",
                    code_name: "Value1",
                },
                sch::EnumValueDef {
                    yass_name: "value-2",
                    code_name: "Value2",
                },
            ],
            unknown_value_name: Some("UnknownValue"),
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-raw-atom",
            code_name: "TestRawAtom",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::RawAtom,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-boolean",
            code_name: "TestBoolean",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Bool,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-int32",
            code_name: "TestInt32",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int32,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-uint32",
            code_name: "TestUInt32",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::UInt32,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-int64",
            code_name: "TestInt64",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Int64,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-uint64",
            code_name: "TestUInt64",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::UInt64,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-float",
            code_name: "TestFloat",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Float,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-byte-string",
            code_name: "TestByteString",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::ByteString,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-utf8-string",
            code_name: "TestUtf8String",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Utf8String,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-ascii-string",
            code_name: "TestAsciiString",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::AsciiString,
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-array",
            code_name: "TestArray",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Array(&sch::Type::RawAtom),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-tuple-empty",
            code_name: "TestTupleEmpty",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Tuple(&[]),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-tuple-two",
            code_name: "TestTupleTwo",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Tuple(&[sch::Type::Bool, sch::Type::Int32]),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-dictionary",
            code_name: "TestDictionary",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Dictionary(&sch::Type::RawAtom),
                },
            ],
        }),
        sch::TypeDef::Struct(sch::StructDef {
            yass_name: "test-boxed",
            code_name: "TestBoxed",
            fields: &[
                sch::StructFieldDef {
                    yass_name: "value",
                    code_name: "value",
                    mode: sch::StructFieldMode::SingleRequired,
                    type_: sch::Type::Boxed(&sch::Type::RawAtom),
                },
            ],
        }),
    ],
};

fn main() {
    let code = yass_codegen::gen_code_for_schema_as_string(&TEST_1_SCHEMA);
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let code_file_path = std::path::Path::new(&out_dir).join("test_1_schema.rs");
    let mut code_file = std::fs::OpenOptions::new()
        .write(true).create(true).truncate(true).open(code_file_path).unwrap();
    
    use std::io::Write;
    code_file.write_all(code.as_bytes()).unwrap();
    code_file.flush().unwrap();
}
