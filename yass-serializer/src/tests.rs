// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod yass_serializer {
    pub use ::*;
}

fn compact_style() -> yass_serializer::SerializeStyle {
    yass_serializer::SerializeStyle::Compact
}

fn spaced_style() -> yass_serializer::SerializeStyle {
    yass_serializer::SerializeStyle::Spaced {
        line_break_type: yass_serializer::LineBreakType::Lf,
        indent_type: yass_serializer::IndentType::Tabulator,
        indent_length: 1,
    }
}

#[test]
fn test_empty() {
    let document = yass_document!(("test"));
    let expected_result_compact = "(test)";
    let expected_result_spaced = "(test)";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_atom() {
    let document = yass_document!(("test") "a": "123");
    let expected_result_compact = "(test) a 123";
    let expected_result_spaced = "(test)\na 123";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_array_1() {
    let document = yass_document!(("test") "a": []);
    let expected_result_compact = "(test) a []";
    let expected_result_spaced = "(test)\na []";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_array_2() {
    let document = yass_document!(("test") "a": ["1", "2", "3"]);
    let expected_result_compact = "(test) a [1 2 3]";
    let expected_result_spaced = "(test)\na [\n\t1\n\t2\n\t3\n]";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_array_3() {
    let document = yass_document!(("test") "a": [[[]]]);
    let expected_result_compact = "(test) a [[[]]]";
    let expected_result_spaced = "(test)\na [\n\t[\n\t\t[]\n\t]\n]";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_struct_1() {
    let document = yass_document!(("test") "a": {});
    let expected_result_compact = "(test) a {}";
    let expected_result_spaced = "(test)\na {}";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_struct_2() {
    let document = yass_document!(("test") "a": {"a": "1", "b": "2", "c": "3"});
    let expected_result_compact = "(test) a {a 1 b 2 c 3}";
    let expected_result_spaced = "(test)\na {\n\ta 1\n\tb 2\n\tc 3\n}";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}

#[test]
fn test_tagged_1() {
    let document = yass_document!(("test") "a": (as "tag": {"a": "1", "b": "2"}));
    let expected_result_compact = "(test) a (tag){a 1 b 2}";
    let expected_result_spaced = "(test)\na (tag){\n\ta 1\n\tb 2\n}";
    let serialized_compact = compact_style().serialize_as_string(&document);
    let serialized_spaced = spaced_style().serialize_as_string(&document);
    assert_eq!(serialized_compact, expected_result_compact);
    assert_eq!(serialized_spaced, expected_result_spaced);
}
