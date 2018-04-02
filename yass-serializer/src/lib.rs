// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! YASS serializer
//!
//! Example
//! -------
//!
//! ```
//! #[macro_use]
//! extern crate yass;
//! extern crate yass_serializer;
//!
//! fn main() {
//!     let document = yass_document!(
//!         ("example")
//!         "key1": r#""value1""#,
//!         "key2": "value2",
//!         "key3": "-1.0"
//!     );
//!     
//!     let serialize_style = yass_serializer::SerializeStyle::Spaced {
//!         line_break_type: yass_serializer::LineBreakType::Lf,
//!         indent_type: yass_serializer::IndentType::Space,
//!         indent_length: 2,
//!     };
//!     let serialized = serialize_style.serialize_as_string(&document);
//!     
//!     // (example)
//!     // key1 "value1"
//!     // key2 value2
//!     // key3 -1.0
//!     let expected = "(example)\nkey1 \"value1\"\nkey2 value2\nkey3 -1.0";
//!     // Note that the serializer won't add a empty line at the end of the output
//!     assert_eq!(serialized, expected);
//! }
//! ```

#[allow(unused_imports)]
#[macro_use]
extern crate yass;

#[cfg(test)]
mod tests;

// Style
#[derive(Clone, Debug)]
pub enum SerializeStyle {
    Compact,
    Spaced {
        line_break_type: LineBreakType,
        indent_type: IndentType,
        indent_length: usize,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineBreakType {
    Lf,
    CrLf,
    Cr,
}

impl LineBreakType {
    pub fn to_str(self) -> &'static str {
        match self {
            LineBreakType::Lf => "\n",
            LineBreakType::CrLf => "\r\n",
            LineBreakType::Cr => "\r",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndentType {
    Space,
    Tabulator,
}

impl IndentType {
    pub fn to_char(self) -> char {
        match self {
            IndentType::Space => ' ',
            IndentType::Tabulator => '\t',
        }
    }
}

impl SerializeStyle {
    pub fn serialize(&self, document: &yass::Document, output: &mut String) {
        match *self {
            SerializeStyle::Compact => {
                CompactSerializer::serialize_root(&document.header, &document.root_fields, output);
            }
            SerializeStyle::Spaced { line_break_type, indent_type, indent_length } => {
                let line_break = line_break_type.to_str();
                let mut indent = String::new();
                indent.push(indent_type.to_char());
                indent = indent.repeat(indent_length);
                SpacedSerializer::new(&line_break, &indent).serialize_root(&document.header, &document.root_fields, output);
            }
        }
    }
    
    #[inline]
    pub fn serialize_as_string(&self, document: &yass::Document) -> String {
        let mut output = String::new();
        self.serialize(document, &mut output);
        output
    }
}

// CompactSerializer
enum CompactSerializer {}

impl CompactSerializer {
    fn serialize_root(header: &str, fields: &[yass::StructField], output: &mut String) {
        output.push('(');
        output.push_str(header);
        output.push(')');
        for field in fields {
            output.push(' ');
            output.push_str(&field.key);
            output.push(' ');
            Self::serialize_value(&field.value, output);
        }
    }
    
    fn serialize_value(value: &yass::Value, output: &mut String) {
        match *value {
            yass::Value::Atom(ref atom) => output.push_str(atom),
            yass::Value::Array(ref array) => {
                output.push('[');
                for (i, item) in array.iter().enumerate() {
                    if i != 0 {
                        output.push(' ');
                    }
                    Self::serialize_value(item, output);
                }
                output.push(']');
            }
            yass::Value::Struct(ref struct_) => {
                output.push('{');
                for (i, field) in struct_.iter().enumerate() {
                    if i != 0 {
                        output.push(' ');
                    }
                    output.push_str(&field.key);
                    output.push(' ');
                    Self::serialize_value(&field.value, output);
                }
                output.push('}');
            }
            yass::Value::Tagged(ref tag, ref sub_value) => {
                output.push('(');
                output.push_str(tag);
                output.push(')');
                Self::serialize_value(sub_value, output);
            }
        }
    }
}

// SpacedSerializer
struct SpacedSerializer<'a, 'b> {
    line_break: &'a str,
    indent: &'b str,
}

impl<'a, 'b> SpacedSerializer<'a, 'b> {
    fn new(line_break: &'a str, indent: &'b str) -> Self {
        Self {
            line_break: line_break,
            indent: indent,
        }
    }
    
    fn serialize_root(&self, header: &str, fields: &[yass::StructField], output: &mut String) {
        output.push('(');
        output.push_str(header);
        output.push(')');
        
        for field in fields {
            output.push_str(self.line_break);
            output.push_str(&field.key);
            output.push(' ');
            self.serialize_value(&field.value, 0, output);
        }
    }
    
    fn serialize_value(&self, value: &yass::Value, depth: usize, output: &mut String) {
        match *value {
            yass::Value::Atom(ref atom) => output.push_str(atom),
            yass::Value::Array(ref array) => {
                if array.len() != 0 {
                    output.push('[');
                    for item in array.iter() {
                        self.write_line_break_and_indent(depth + 1, output);
                        self.serialize_value(item, depth + 1, output);
                    }
                    self.write_line_break_and_indent(depth, output);
                    output.push(']');
                } else {
                    output.push_str("[]");
                }
            }
            yass::Value::Struct(ref struct_) => {
                if struct_.len() != 0 {
                    output.push('{');
                    for field in struct_.iter() {
                        self.write_line_break_and_indent(depth + 1, output);
                        output.push_str(&field.key);
                        output.push(' ');
                        self.serialize_value(&field.value, depth + 1, output);
                    }
                    self.write_line_break_and_indent(depth, output);
                    output.push('}');
                } else {
                    output.push_str("{}");
                }
            }
            yass::Value::Tagged(ref tag, ref sub_value) => {
                output.push('(');
                output.push_str(tag);
                output.push(')');
                self.serialize_value(sub_value, depth, output);
            }
        }
    }
    
    fn write_line_break_and_indent(&self, depth: usize, output: &mut String) {
        output.push_str(self.line_break);
        for _ in 0 .. depth {
            output.push_str(self.indent);
        }
    }
}
