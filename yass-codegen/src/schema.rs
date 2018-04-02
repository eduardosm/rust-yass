// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[derive(Clone, Debug)]
pub struct Schema<'a> {
    pub header: &'a str,
    pub root_type: &'a str,
    pub type_defs: &'a [TypeDef<'a>],
}

#[derive(Clone, Debug)]
pub enum TypeDef<'a> {
    Struct(StructDef<'a>),
    TaggedUnion(TaggedUnionDef<'a>),
    Enum(EnumDef<'a>),
}

#[derive(Clone, Debug)]
pub struct StructDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
    pub fields: &'a [StructFieldDef<'a>],
}

#[derive(Clone, Debug)]
pub struct StructFieldDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
    pub mode: StructFieldMode,
    pub type_: Type<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StructFieldMode {
    SingleOptional,
    SingleRequired,
    MultipleOptional,
    MultipleRequired,
}

#[derive(Clone, Debug)]
pub struct TaggedUnionDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
    pub variants: &'a [VariantDef<'a>],
}

#[derive(Clone, Debug)]
pub struct VariantDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
    pub type_: Type<'a>,
}

#[derive(Clone, Debug)]
pub struct EnumDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
    pub values: &'a [EnumValueDef<'a>],
    pub unknown_value_name: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct EnumValueDef<'a> {
    pub yass_name: &'a str,
    pub code_name: &'a str,
}

#[derive(Clone, Debug)]
pub enum Type<'a> {
    RawAtom,
    Bool,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    ByteString,
    Utf8String,
    AsciiString,
    Array(&'a Type<'a>),
    Tuple(&'a [Type<'a>]),
    Dictionary(&'a Type<'a>),
    Defined(&'a str),
    Boxed(&'a Type<'a>),
}
