// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate yass;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidDocumentHeader {
        header: String,
    },
    
    InvalidValueTypeForStruct {
        struct_name: String,
        value_pos: Option<yass::Pos>,
    },
    UnknownStructField {
        struct_name: String,
        field_name: String,
        field_pos: Option<yass::Pos>,
    },
    MissingStructField {
        struct_name: String,
        field_name: String,
        struct_pos: Option<yass::Pos>,
    },
    RepeatedStructField {
        struct_name: String,
        field_name: String,
        field_pos: Option<yass::Pos>,
    },
    
    InvalidValueTypeForTaggedUnion {
        tagged_union_name: String,
        value_pos: Option<yass::Pos>,
    },
    UnknownTaggedUnionVariant {
        tagged_union_name: String,
        variant_name: String,
        value_pos: Option<yass::Pos>,
    },
    
    InvalidValueTypeForEnum {
        enum_name: String,
        value_pos: Option<yass::Pos>,
    },
    UnknownEnumValue {
        enum_name: String,
        value_name: String,
        value_pos: Option<yass::Pos>,
    },
    
    InvalidValueTypeForRawAtom {
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForBool {
        value_pos: Option<yass::Pos>,
    },
    InvalidBoolValue {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForInt32 {
        value_pos: Option<yass::Pos>,
    },
    InvalidInt32Value {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForUInt32 {
        value_pos: Option<yass::Pos>,
    },
    InvalidUInt32Value {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForInt64 {
        value_pos: Option<yass::Pos>,
    },
    InvalidInt64Value {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForUInt64 {
        value_pos: Option<yass::Pos>,
    },
    InvalidUInt64Value {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForFloat {
        value_pos: Option<yass::Pos>,
    },
    InvalidFloatValue {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForByteString {
        value_pos: Option<yass::Pos>,
    },
    InvalidByteStringValue {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForUtf8String {
        value_pos: Option<yass::Pos>,
    },
    InvalidUtf8StringValue {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForAsciiString {
        value_pos: Option<yass::Pos>,
    },
    InvalidAsciiStringValue {
        value: String,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForArray {
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForTuple {
        value_pos: Option<yass::Pos>,
    },
    InvalidNumberOfTupleElements {
        num_elements: usize,
        num_expected: usize,
        value_pos: Option<yass::Pos>,
    },
    InvalidValueTypeForDictionary {
        value_pos: Option<yass::Pos>,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidDocumentHeader { ref header } => {
                write!(f, "Invalid document header {:?}", header)?;
                Ok(())
            }
            
            Error::InvalidValueTypeForStruct { ref struct_name, value_pos } => {
                write!(f, "Invalid value type for struct {:?}", struct_name)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::UnknownStructField { ref struct_name, ref field_name, field_pos } => {
                write!(f, "Unknown field {:?} in struct {:?}", field_name, struct_name)?;
                if let Some(field_pos) = field_pos {
                    write!(f, " at {}:{}", field_pos.line + 1, field_pos.column + 1)?;
                }
                Ok(())
            }
            Error::MissingStructField { ref struct_name, ref field_name, struct_pos } => {
                write!(f, "Missing field {:?} in struct {:?}", field_name, struct_name)?;
                if let Some(struct_pos) = struct_pos {
                    write!(f, " at {}:{}", struct_pos.line + 1, struct_pos.column + 1)?;
                }
                Ok(())
            }
            Error::RepeatedStructField { ref struct_name, ref field_name, field_pos } => {
                write!(f, "Repeated field {:?} in struct {:?}", field_name, struct_name)?;
                if let Some(field_pos) = field_pos {
                    write!(f, " at {}:{}", field_pos.line + 1, field_pos.column + 1)?;
                }
                Ok(())
            }
            
            Error::InvalidValueTypeForTaggedUnion { ref tagged_union_name, value_pos } => {
                write!(f, "Invalid value type for tagged union {:?}", tagged_union_name)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::UnknownTaggedUnionVariant { ref tagged_union_name, ref variant_name, value_pos } => {
                write!(f, "Unknown variant {:?} for tagged union {:?}", variant_name, tagged_union_name)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            
            Error::InvalidValueTypeForEnum { ref enum_name, value_pos } => {
                write!(f, "Invalid value type for enum {:?}", enum_name)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::UnknownEnumValue { ref enum_name, ref value_name, value_pos } => {
                write!(f, "Unknown value {:?} for enum {:?}", value_name, enum_name)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            
            Error::InvalidValueTypeForRawAtom { value_pos } => {
                f.write_str("Invalid value type for raw atom")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForBool { value_pos } => {
                f.write_str("Invalid value type for boolean")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidBoolValue { ref value, value_pos } => {
                write!(f, "Invalid boolean value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForInt32 { value_pos } => {
                f.write_str("Invalid value type for signed 32-bit integer")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidInt32Value { ref value, value_pos } => {
                write!(f, "Invalid signed 32-bit integer value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForUInt32 { value_pos } => {
                f.write_str("Invalid value type for unsigned 32-bit integer")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidUInt32Value { ref value, value_pos } => {
                write!(f, "Invalid unsigned 32-bit integer value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForInt64 { value_pos } => {
                f.write_str("Invalid value type for signed 64-bit integer")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidInt64Value { ref value, value_pos } => {
                write!(f, "Invalid signed 64-bit integer value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForUInt64 { value_pos } => {
                f.write_str("Invalid value type for unsigned 64-bit integer")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidUInt64Value { ref value, value_pos } => {
                write!(f, "Invalid unsigned 64-bit integer value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForFloat { value_pos } => {
                f.write_str("Invalid value type for floating point")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidFloatValue { ref value, value_pos } => {
                write!(f, "Invalid floating point value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForByteString { value_pos } => {
                f.write_str("Invalid value type for byte string")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidByteStringValue { ref value, value_pos } => {
                write!(f, "Invalid byte string value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForUtf8String { value_pos } => {
                f.write_str("Invalid value type for UTF-8 string")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidUtf8StringValue { ref value, value_pos } => {
                write!(f, "Invalid UTF-8 string value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForAsciiString { value_pos } => {
                f.write_str("Invalid value type for ASCII string")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidAsciiStringValue { ref value, value_pos } => {
                write!(f, "Invalid ASCII string value {:?}", value)?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForArray { value_pos } => {
                f.write_str("Invalid value type for array")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidValueTypeForTuple { value_pos } => {
                f.write_str("Invalid value type for tuple")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
            Error::InvalidNumberOfTupleElements { num_elements, num_expected, value_pos } => {
                f.write_str("Invalid number of tuple elements")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                write!(f, ", expected {}, found {}", num_expected, num_elements)?;
                Ok(())
            }
            Error::InvalidValueTypeForDictionary { value_pos } => {
                f.write_str("Invalid value type for dictionary")?;
                if let Some(value_pos) = value_pos {
                    write!(f, " at {}:{}", value_pos.line + 1, value_pos.column + 1)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidDocumentHeader { .. } => "Invalid document header",
            
            Error::InvalidValueTypeForStruct { .. } => "Invalid value type for struct",
            Error::UnknownStructField { .. } => "Unknown struct field",
            Error::MissingStructField { .. } => "Missing struct field",
            Error::RepeatedStructField { .. } => "Repeated struct field",
            
            Error::InvalidValueTypeForTaggedUnion { .. } => "Invalid value type for tagged union",
            Error::UnknownTaggedUnionVariant { .. } => "Invalid tagged union variant",
            
            Error::InvalidValueTypeForEnum { .. } => "Invalid value type for enum",
            Error::UnknownEnumValue { .. } => "Unknown enum value",
            
            Error::InvalidValueTypeForRawAtom { .. } => "Invalid value type for raw atom",
            Error::InvalidValueTypeForBool { .. } => "Invalid value type for boolean",
            Error::InvalidBoolValue { .. } => "Invalid boolean value",
            Error::InvalidValueTypeForInt32 { .. } => "Invalid value type for signed 32-bit integer",
            Error::InvalidInt32Value { .. } => "Invalid signed 32-bit integer value",
            Error::InvalidValueTypeForUInt32 { .. } => "Invalid value type for unsigned 32-bit integer",
            Error::InvalidUInt32Value { .. } => "Invalid unsigned 32-bit integer value",
            Error::InvalidValueTypeForInt64 { .. } => "Invalid value type for signed 64-bit integer",
            Error::InvalidInt64Value { .. } => "Invalid signed 64-bit integer value",
            Error::InvalidValueTypeForUInt64 { .. } => "Invalid value type for unsigned 64-bit integer",
            Error::InvalidUInt64Value { .. } => "Invalid unsigned 64-bit integer value",
            Error::InvalidValueTypeForFloat { .. } => "Invalid value type for floating point",
            Error::InvalidFloatValue { .. } => "Invalid floating point value",
            Error::InvalidValueTypeForByteString { .. } => "Invalid value type for byte string",
            Error::InvalidByteStringValue { .. } => "Invalid byte string value",
            Error::InvalidValueTypeForUtf8String { .. } => "Invalid value type for UTF-8 string",
            Error::InvalidUtf8StringValue { .. } => "Invalid UTF-8 string value",
            Error::InvalidValueTypeForAsciiString { .. } => "Invalid value type for ASCII string",
            Error::InvalidAsciiStringValue { .. } => "Invalid ASCII string value",
            Error::InvalidValueTypeForArray { .. } => "Invalid value type for array",
            Error::InvalidValueTypeForTuple { .. } => "Invalid value type for tuple",
            Error::InvalidNumberOfTupleElements { .. } => "Invalid number of tuple elements",
            Error::InvalidValueTypeForDictionary { .. } => "Invalid value type for dictionary",
        }
    }
}
