// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use yass;
mod yass_parser {
    pub use ::*;
}

macro_rules! pos_array {
    [$(($line:expr, $column:expr)),*] => {
        [$(yass::Pos::new($line, $column)),*]
    }
}

trait GatherPositions {
    fn gather_positions(&self, pos_map: &yass::PosMap, dst: &mut Vec<yass::Pos>);
    
    fn gather_positions_to_vec(&self, pos_map: &yass::PosMap) -> Vec<yass::Pos> {
        let mut positions = Vec::new();
        self.gather_positions(pos_map, &mut positions);
        positions
    }
}

impl GatherPositions for yass::Value {
    fn gather_positions(&self, pos_map: &yass::PosMap, dst: &mut Vec<yass::Pos>) {
        dst.push(pos_map.get_value_pos(self).unwrap());
        match *self {
            yass::Value::Atom(_) => {}
            yass::Value::Array(ref array) => {
                for item in array {
                    item.gather_positions(pos_map, dst);
                }
            }
            yass::Value::Struct(ref fields) => {
                for field in fields {
                    field.gather_positions(pos_map, dst);
                }
            }
            yass::Value::Tagged(_, ref sub_value) => {
                sub_value.gather_positions(pos_map, dst);
            }
        }
    }
}

impl GatherPositions for yass::StructField {
    fn gather_positions(&self, pos_map: &yass::PosMap, dst: &mut Vec<yass::Pos>) {
        dst.push(pos_map.get_struct_field_pos(&self.value).unwrap());
        self.value.gather_positions(pos_map, dst);
    }
}

impl GatherPositions for yass::Document {
    fn gather_positions(&self, pos_map: &yass::PosMap, dst: &mut Vec<yass::Pos>) {
        for field in self.root_fields.iter() {
            field.gather_positions(pos_map, dst);
        }
    }
}

#[test]
fn test_simple_1() {
    let src_data = b"(test)";
    let expected_document = yass_document!(("test"));
    let expected_positions = pos_array![];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_simple_2() {
    let src_data = b"(test) a b";
    let expected_document = yass_document!(("test") "a": "b");
    let expected_positions = pos_array![(0, 7), (0, 9)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_simple_stream() {
    let mut src_data = &b"(test)"[..];
    let expected_document = yass_document!(("test"));
    let expected_positions = pos_array![];
    let (_, result_doc, pos_map) = yass_parser::parse_stream(yass_parser::ParserLimits::unlimited(), &mut src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_simple_3() {
    let src_data = b"(test) a1 b1 a2 b2";
    let expected_document = yass_document!(("test") "a1": "b1", "a2": "b2");
    let expected_positions = pos_array![(0, 7), (0, 10), (0, 13), (0, 16)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_array_1() {
    let src_data = b"(test) array []";
    let expected_document = yass_document!(("test") "array": []);
    let expected_positions = pos_array![(0, 7), (0, 13)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_array_2() {
    let src_data = b"(test) array [1 2 3]";
    let expected_document = yass_document!(("test") "array": ["1", "2", "3"]);
    let expected_positions = pos_array![(0, 7), (0, 13), (0, 14), (0, 16), (0, 18)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_struct_1() {
    let src_data = b"(test) struct {}";
    let expected_document = yass_document!(("test") "struct": {});
    let expected_positions = pos_array![(0, 7), (0, 14)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_struct_2() {
    let src_data = b"(test) struct {a1 b1 a2 b2}";
    let expected_document = yass_document!(("test") "struct": {"a1": "b1", "a2": "b2"});
    let expected_positions = pos_array![(0, 7), (0, 14), (0, 15), (0, 18), (0, 21), (0, 24)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_string_1() {
    let src_data = b"(test) a \"abc\"";
    let expected_document = yass_document!(("test") "a": "\"abc\"");
    let expected_positions = pos_array![(0, 7), (0, 9)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_string_2() {
    let src_data = b"(test) a A\"a b c\"B";
    let expected_document = yass_document!(("test") "a": "A\"a b c\"B");
    let expected_positions = pos_array![(0, 7), (0, 9)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_tagged_1() {
    let src_data = b"(test) a (tag)123";
    let expected_document = yass_document!(("test") "a": (as "tag": "123"));
    let expected_positions = pos_array![(0, 7), (0, 9), (0, 14)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_struct_string_3() {
    let src_data = b"(test) a \" \\\\ \\\" \"";
    let expected_document = yass_document!(("test") "a": "\" \\\\ \\\" \"");
    let expected_positions = pos_array![(0, 7), (0, 9)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_multiline_lf() {
    let src_data = b"(test) a b\nc d";
    let expected_document = yass_document!(("test") "a": "b", "c": "d");
    let expected_positions = pos_array![(0, 7), (0, 9), (1, 0), (1, 2)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_multiline_crlf() {
    let src_data = b"(test) a b\r\nc d";
    let expected_document = yass_document!(("test") "a": "b", "c": "d");
    let expected_positions = pos_array![(0, 7), (0, 9), (1, 0), (1, 2)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_multiline_cr() {
    let src_data = b"(test) a b\rc d";
    let expected_document = yass_document!(("test") "a": "b", "c": "d");
    let expected_positions = pos_array![(0, 7), (0, 9), (1, 0), (1, 2)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_multiline_mixed() {
    let src_data = b"(test) a b\n\r\r\nc d";
    let expected_document = yass_document!(("test") "a": "b", "c": "d");
    let expected_positions = pos_array![(0, 7), (0, 9), (3, 0), (3, 2)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_comment_1() {
    let src_data = b"(test) a b\\comment";
    let expected_document = yass_document!(("test") "a": "b");
    let expected_positions = pos_array![(0, 7), (0, 9)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_comment_2() {
    let src_data = b"(test) a b\\comment\nc d";
    let expected_document = yass_document!(("test") "a": "b", "c": "d");
    let expected_positions = pos_array![(0, 7), (0, 9), (1, 0), (1, 2)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_comment_3() {
    let src_data = b"(test) a [1\\comment\n2]";
    let expected_document = yass_document!(("test") "a": ["1", "2"]);
    let expected_positions = pos_array![(0, 7), (0, 9), (0, 10), (1, 0)];
    let (result_doc, pos_map) = yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).unwrap();
    assert_eq!(result_doc, expected_document);
    assert_eq!(result_doc.gather_positions_to_vec(&pos_map), expected_positions);
}

#[test]
fn test_fail_empty() {
    let src_data = b"";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_empty_head() {
    let src_data = b"()";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_unfinished_string() {
    let src_data = b"(test) \"abc";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_unclosed_array() {
    let src_data = b"(test) [1 2";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_unclosed_struct() {
    let src_data = b"(test) {a 1 b 2";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_unclosed_array_with_comment() {
    let src_data = b"(test) [\\comment]";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_illegal_chr_in_atom() {
    let src_data = b"(test) a A\xFFB";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_illegal_chr_in_string() {
    let src_data = b"(test) a \"A\xFFB\"";
    assert!(yass_parser::parse(yass_parser::ParserLimits::unlimited(), src_data).is_err());
}

#[test]
fn test_fail_too_deep() {
    let src_data = b"(test) a [[]]";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_depth = 1;
    let expected_error = yass_parser::ParserError::TooDeep { pos: yass::Pos::new(0, 10) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}

#[test]
fn test_fail_atom_too_long() {
    let src_data = b"(test) abc 123";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_atom_length = 2;
    let expected_error = yass_parser::ParserError::AtomTooLong { pos: yass::Pos::new(0, 11) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}

#[test]
fn test_fail_tag_too_long() {
    let src_data = b"(test) abc (tag)123";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_tag_length = 2;
    let expected_error = yass_parser::ParserError::TagTooLong { pos: yass::Pos::new(0, 12) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}

#[test]
fn test_fail_key_too_long() {
    let src_data = b"(test) abc []";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_key_length = 2;
    let expected_error = yass_parser::ParserError::KeyTooLong { pos: yass::Pos::new(0, 7) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}

#[test]
fn test_fail_array_too_big() {
    let src_data = b"(test) a [1 2 3]";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_array_size = 2;
    let expected_error = yass_parser::ParserError::ArrayTooBig { pos: yass::Pos::new(0, 14) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}

#[test]
fn test_fail_struct_too_big() {
    let src_data = b"(test) a {a 1 b 2 c 3}";
    let mut limits = yass_parser::ParserLimits::unlimited();
    limits.max_struct_size = 2;
    let expected_error = yass_parser::ParserError::StructTooBig { pos: yass::Pos::new(0, 18) };
    assert_eq!(yass_parser::parse(limits, src_data).unwrap_err(), expected_error);
}
