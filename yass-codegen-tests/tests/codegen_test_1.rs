// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate yass;
extern crate yass_aux;
extern crate yass_schema_error;

mod gen {
    use yass;
    use yass_aux;
    use yass_schema_error;
    
    include!(concat!(env!("OUT_DIR"), "/test_1_schema.rs"));
}

use yass_schema_error::Error as SchError;

// document
#[test]
fn test_document() {
    let data_sch = gen::EmptyStruct {};
    let data_yass = yass_document!(("test"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_document(), data_yass);
    assert_eq!(gen::EmptyStruct::from_yass_document(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_document_fail_invalid_header() {
    let data_yass = yass_document!(("test2"));
    let expected_error = SchError::InvalidDocumentHeader {
        header: "test2".to_string(),
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EmptyStruct::from_yass_document(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_document_fail_unknown_field() {
    let data_yass = yass_document!(("test") "field": "test");
    let expected_error = SchError::UnknownStructField {
        struct_name: "empty-struct".to_string(),
        field_name: "field".to_string(),
        field_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EmptyStruct::from_yass_document(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// struct
#[test]
fn test_struct_empty() {
    let data_sch = gen::EmptyStruct {};
    let data_yass = yass_value!({});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EmptyStruct::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_empty_fail_invalid_type() {
    let data_yass = yass_value!([]);
    let expected_error = SchError::InvalidValueTypeForStruct {
        struct_name: "empty-struct".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EmptyStruct::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_empty_fail_unknown_field() {
    let data_yass = yass_value!({"field": "test"});
    let expected_error = SchError::UnknownStructField {
        struct_name: "empty-struct".to_string(),
        field_name: "field".to_string(),
        field_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EmptyStruct::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_with_single_required() {
    let data_sch = gen::StructWithSingleRequired { field: "test".to_string() };
    let data_yass = yass_value!({"field": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithSingleRequired::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_single_required_fail_missing() {
    let data_yass = yass_value!({});
    let expected_error = SchError::MissingStructField {
        struct_name: "struct-with-single-required".to_string(),
        field_name: "field".to_string(),
        struct_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::StructWithSingleRequired::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_with_single_required_fail_repeated() {
    let data_yass = yass_value!({"field": "test1", "field": "test2"});
    let expected_error = SchError::RepeatedStructField {
        struct_name: "struct-with-single-required".to_string(),
        field_name: "field".to_string(),
        field_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::StructWithSingleRequired::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_with_single_optional_with() {
    let data_sch = gen::StructWithSingleOptional { field: Some("test".to_string()) };
    let data_yass = yass_value!({"field": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithSingleOptional::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_single_optional_without() {
    let data_sch = gen::StructWithSingleOptional { field: None };
    let data_yass = yass_value!({});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithSingleOptional::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_single_optional_fail_repeated() {
    let data_yass = yass_value!({"field": "test1", "field": "test2"});
    let expected_error = SchError::RepeatedStructField {
        struct_name: "struct-with-single-optional".to_string(),
        field_name: "field".to_string(),
        field_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::StructWithSingleOptional::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_with_multiple_required_one() {
    let data_sch = gen::StructWithMultipleRequired { field: vec!["test".to_string()] };
    let data_yass = yass_value!({"field": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithMultipleRequired::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_multiple_required_two() {
    let data_sch = gen::StructWithMultipleRequired { field: vec!["test1".to_string(), "test2".to_string()] };
    let data_yass = yass_value!({"field": "test1", "field": "test2"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithMultipleRequired::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_multiple_required_fail_missing() {
    let data_yass = yass_value!({});
    let expected_error = SchError::MissingStructField {
        struct_name: "struct-with-multiple-required".to_string(),
        field_name: "field".to_string(),
        struct_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::StructWithMultipleRequired::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_struct_with_multiple_optional_zero() {
    let data_sch = gen::StructWithMultipleOptional { field: vec![] };
    let data_yass = yass_value!({});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithMultipleOptional::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_multiple_optional_one() {
    let data_sch = gen::StructWithMultipleOptional { field: vec!["test".to_string()] };
    let data_yass = yass_value!({"field": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithMultipleOptional::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_struct_with_multiple_optional_two() {
    let data_sch = gen::StructWithMultipleOptional { field: vec!["test1".to_string(), "test2".to_string()] };
    let data_yass = yass_value!({"field": "test1", "field": "test2"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::StructWithMultipleOptional::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

// tagged union
#[test]
fn test_tagged_union_variant_1() {
    let data_sch = gen::TaggedUnion1::Variant1(0);
    let data_yass = yass_value!((as "variant-1": "0"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_tagged_union_variant_2() {
    let data_sch = gen::TaggedUnion1::Variant2(false);
    let data_yass = yass_value!((as "variant-2": "false"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_tagged_union_fail_invalid_type() {
    let data_yass = yass_value!([]);
    let expected_error = SchError::InvalidValueTypeForTaggedUnion {
        tagged_union_name: "tagged-union-1".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tagged_union_fail_unknown_variant() {
    let data_yass = yass_value!((as "variant-3": "test"));
    let expected_error = SchError::UnknownTaggedUnionVariant {
        tagged_union_name: "tagged-union-1".to_string(),
        variant_name: "variant-3".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tagged_union_fail_invalid_item_type_1() {
    let data_yass = yass_value!((as "variant-1": []));
    let expected_error = SchError::InvalidValueTypeForInt32 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tagged_union_fail_invalid_item_type_2() {
    let data_yass = yass_value!((as "variant-2": []));
    let expected_error = SchError::InvalidValueTypeForBool {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TaggedUnion1::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// enum
#[test]
fn test_enum_without_unknown_1() {
    let data_sch = gen::EnumWithoutUnknown::Value1;
    let data_yass = yass_value!(("value-1"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EnumWithoutUnknown::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_enum_without_unknown_2() {
    let data_sch = gen::EnumWithoutUnknown::Value2;
    let data_yass = yass_value!(("value-2"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EnumWithoutUnknown::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_enum_without_unknown_fail_invalid_type() {
    let data_yass = yass_value!([]);
    let expected_error = SchError::InvalidValueTypeForEnum {
        enum_name: "enum-without-unknown".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EnumWithoutUnknown::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_enum_without_unknown_fail_unknown_value() {
    let data_yass = yass_value!(("value-3"));
    let expected_error = SchError::UnknownEnumValue {
        enum_name: "enum-without-unknown".to_string(),
        value_name: "value-3".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EnumWithoutUnknown::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_enum_with_unknown_1() {
    let data_sch = gen::EnumWithUnknown::Value1;
    let data_yass = yass_value!(("value-1"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EnumWithUnknown::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_enum_with_unknown_2() {
    let data_sch = gen::EnumWithUnknown::Value2;
    let data_yass = yass_value!(("value-2"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EnumWithUnknown::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_enum_with_unknown_unknown() {
    let data_sch = gen::EnumWithUnknown::UnknownValue("value-3".to_string());
    let data_yass = yass_value!(("value-3"));
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::EnumWithUnknown::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_enum_with_unknown_fail_invalid_type() {
    let data_yass = yass_value!([]);
    let expected_error = SchError::InvalidValueTypeForEnum {
        enum_name: "enum-with-unknown".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::EnumWithUnknown::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// raw atom
#[test]
fn test_raw_atom() {
    let data_sch = gen::TestRawAtom { value: "test".to_string() };
    let data_yass = yass_value!({"value": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestRawAtom::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_raw_atom_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForRawAtom {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestRawAtom::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// boolean
#[test]
fn test_boolean_true() {
    let data_sch = gen::TestBoolean { value: true };
    let data_yass = yass_value!({"value": "true"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestBoolean::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_boolean_false() {
    let data_sch = gen::TestBoolean { value: false };
    let data_yass = yass_value!({"value": "false"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestBoolean::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_boolean_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForBool {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestBoolean::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_boolean_fail_invalid_value() {
    let data_yass = yass_value!({"value": "abc"});
    let expected_error = SchError::InvalidBoolValue {
        value: "abc".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestBoolean::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// int32
#[test]
fn test_int32() {
    let data_sch = gen::TestInt32 { value: -12345678 };
    let data_yass = yass_value!({"value": "-12345678"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestInt32::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_int32_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForInt32 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestInt32::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_int32_fail_invalid_value() {
    let data_yass = yass_value!({"value": "123456789123456789"});
    let expected_error = SchError::InvalidInt32Value {
        value: "123456789123456789".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestInt32::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// uint32
#[test]
fn test_uint32() {
    let data_sch = gen::TestUInt32 { value: 12345678 };
    let data_yass = yass_value!({"value": "12345678"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestUInt32::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_uint32_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForUInt32 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUInt32::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_uint32_fail_invalid_value() {
    let data_yass = yass_value!({"value": "123456789123456789"});
    let expected_error = SchError::InvalidUInt32Value {
        value: "123456789123456789".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUInt32::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// int64
#[test]
fn test_int64() {
    let data_sch = gen::TestInt64 { value: -1234567812345678 };
    let data_yass = yass_value!({"value": "-1234567812345678"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestInt64::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_int64_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForInt64 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestInt64::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_int64_fail_invalid_value() {
    let data_yass = yass_value!({"value": "123456789123456789123456789"});
    let expected_error = SchError::InvalidInt64Value {
        value: "123456789123456789123456789".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestInt64::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// uint64
#[test]
fn test_uint64() {
    let data_sch = gen::TestUInt64 { value: 1234567812345678 };
    let data_yass = yass_value!({"value": "1234567812345678"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestUInt64::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_uint64_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForUInt64 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUInt64::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_uint64_fail_invalid_value() {
    let data_yass = yass_value!({"value": "123456789123456789123456789"});
    let expected_error = SchError::InvalidUInt64Value {
        value: "123456789123456789123456789".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUInt64::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// float
#[test]
fn test_float() {
    let data_sch = gen::TestFloat { value: 1.234 };
    let data_yass = yass_value!({"value": "1.234"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestFloat::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_float_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForFloat {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestFloat::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_float_fail_invalid_value() {
    let data_yass = yass_value!({"value": "1.234e"});
    let expected_error = SchError::InvalidFloatValue {
        value: "1.234e".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestFloat::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// byte string
#[test]
fn test_byte_string() {
    let data_sch = gen::TestByteString { value: b"test\xFF".to_vec() };
    let data_yass = yass_value!({"value": "\"test\\xff\""});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestByteString::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_byte_string_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForByteString {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestByteString::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_byte_string_fail_invalid_value() {
    let data_yass = yass_value!({"value": "\"12\\u{0}34\""});
    let expected_error = SchError::InvalidByteStringValue {
        value: "\"12\\u{0}34\"".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestByteString::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// utf8 string
#[test]
fn test_utf8_string() {
    let data_sch = gen::TestUtf8String { value: "test\u{FFFD}".to_string() };
    let data_yass = yass_value!({"value": "\"test\\u{fffd}\""});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestUtf8String::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_utf8_string_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForUtf8String {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUtf8String::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_utf8_string_fail_invalid_value() {
    let data_yass = yass_value!({"value": "\"12\\xFF34\""});
    let expected_error = SchError::InvalidUtf8StringValue {
        value: "\"12\\xFF34\"".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestUtf8String::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// ascii string
#[test]
fn test_ascii_string() {
    let data_sch = gen::TestAsciiString { value: "test\x00".to_string() };
    let data_yass = yass_value!({"value": "\"test\\x00\""});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestAsciiString::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_ascii_string_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForAsciiString {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestAsciiString::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_ascii_string_fail_invalid_value() {
    let data_yass = yass_value!({"value": "\"12\\xFF34\""});
    let expected_error = SchError::InvalidAsciiStringValue {
        value: "\"12\\xFF34\"".to_string(),
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestAsciiString::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// array
#[test]
fn test_array_empty() {
    let data_sch = gen::TestArray { value: vec![] };
    let data_yass = yass_value!({"value": []});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_array_one() {
    let data_sch = gen::TestArray { value: vec!["test".to_string()] };
    let data_yass = yass_value!({"value": ["test"]});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_array_two() {
    let data_sch = gen::TestArray { value: vec!["test1".to_string(), "test2".to_string()] };
    let data_yass = yass_value!({"value": ["test1", "test2"]});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_array_fail_invalid_type() {
    let data_yass = yass_value!({"value": "abc"});
    let expected_error = SchError::InvalidValueTypeForArray {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_array_fail_invalid_item_type_1() {
    let data_yass = yass_value!({"value": [[], "test"]});
    let expected_error = SchError::InvalidValueTypeForRawAtom {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_array_fail_invalid_item_type_2() {
    let data_yass = yass_value!({"value": ["test", []]});
    let expected_error = SchError::InvalidValueTypeForRawAtom {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestArray::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// tuple
#[test]
fn test_tuple_empty() {
    let data_sch = gen::TestTupleEmpty { value: () };
    let data_yass = yass_value!({"value": []});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestTupleEmpty::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_tuple_empty_fail_invalid_type() {
    let data_yass = yass_value!({"value": "123"});
    let expected_error = SchError::InvalidValueTypeForTuple {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestTupleEmpty::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tuple_empty_fail_invalid_num_elements() {
    let data_yass = yass_value!({"value": ["a", "b"]});
    let expected_error = SchError::InvalidNumberOfTupleElements {
        num_elements: 2,
        num_expected: 0,
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestTupleEmpty::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tuple_two() {
    let data_sch = gen::TestTupleTwo { value: (false, 0) };
    let data_yass = yass_value!({"value": ["false", "0"]});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestTupleTwo::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_tuple_two_fail_invalid_num_elements() {
    let data_yass = yass_value!({"value": ["false"]});
    let expected_error = SchError::InvalidNumberOfTupleElements {
        num_elements: 1,
        num_expected: 2,
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestTupleTwo::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tuple_two_fail_invalid_item_type_1() {
    let data_yass = yass_value!({"value": [[], "0"]});
    let expected_error = SchError::InvalidValueTypeForBool {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestTupleTwo::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_tuple_two_fail_invalid_item_type_2() {
    let data_yass = yass_value!({"value": ["false", []]});
    let expected_error = SchError::InvalidValueTypeForInt32 {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestTupleTwo::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// dictionary
#[test]
fn test_dictionary_empty() {
    let data_sch = gen::TestDictionary { value: vec![] };
    let data_yass = yass_value!({"value": {}});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestDictionary::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_dictionary_two() {
    let data_sch = gen::TestDictionary { value: vec![("a".to_string(), "test1".to_string()), ("b".to_string(), "test2".to_string())] };
    let data_yass = yass_value!({"value": {"a": "test1", "b": "test2"}});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestDictionary::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_dictionary_fail_invalid_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForDictionary {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestDictionary::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

#[test]
fn test_dictionary_fail_invalid_item_type() {
    let data_yass = yass_value!({"value": {"a": []}});
    let expected_error = SchError::InvalidValueTypeForRawAtom {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestDictionary::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}

// boxed
#[test]
fn test_boxed() {
    let data_sch = gen::TestBoxed { value: Box::new("test".to_string()) };
    let data_yass = yass_value!({"value": "test"});
    let pos_map = yass::PosMap::new();
    
    assert_eq!(data_sch.to_yass_value(), *data_yass);
    assert_eq!(gen::TestBoxed::from_yass_value(&data_yass, &pos_map).unwrap(), data_sch);
}

#[test]
fn test_boxed_fail_invalid_item_type() {
    let data_yass = yass_value!({"value": []});
    let expected_error = SchError::InvalidValueTypeForRawAtom {
        value_pos: None,
    };
    let pos_map = yass::PosMap::new();
    
    assert_eq!(gen::TestBoxed::from_yass_value(&data_yass, &pos_map).unwrap_err(), expected_error);
}
