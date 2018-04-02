// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! YASS is a data and configuration text format.
//!
//! Example
//! -------
//!
//! ```text
//! \ Based on 2nd example at https://json.org/example.html (as of 2018-03-31)
//! (example-1-widget)
//! debug true
//! window {
//!   title "Sample Konfabulator Widget"
//!   name "main_window"
//!   width 500
//!   height 500
//! }
//! item (image){
//!   src "Images/Sun.png"
//!   name "sun1"
//!   hoffset 250
//!   voffset 250
//!   alignment center
//! }
//! item (text){
//!   data "Click Here"
//!   size 36
//!   style bold
//!   name "text1"
//!   hoffset 250
//!   voffset 100
//!   alignment center
//!   on-mouse-up "sun1.opacity = (sun1.opacity / 100) * 90;"
//! }
//! ```

use std::collections::HashMap;

/// Represents a position in a text file.
///
/// `line` and `column` begin to count with zero.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl Pos {
    #[inline]
    pub fn new(line: u32, column: u32) -> Self {
        Self { line: line, column: column }
    }
}

/// Maps YASS values with their positions in the original
/// text file.
#[derive(Debug)]
pub struct PosMap {
    values: HashMap<*const Value, Pos>,
    struct_fields: HashMap<*const Value, Pos>,
}

impl PosMap {
    /// Creates an empty map.
    #[inline]
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            struct_fields: HashMap::new(),
        }
    }
    
    /// Gets the position of a value, returning `None` if unknown.
    #[inline]
    pub fn get_value_pos(&self, value: &Value) -> Option<Pos> {
        self.values.get(&(value as *const Value)).map(|&v| v)
    }
    
    /// Gets the position of a struct field, returning `None` if unknown.
    #[inline]
    pub fn get_struct_field_pos(&self, value: &Value) -> Option<Pos> {
        self.struct_fields.get(&(value as *const Value)).map(|&v| v)
    }
    
    /// Adds or overrides the position of a value.
    #[inline]
    pub fn set_value_pos(&mut self, value: &Value, pos: Pos) {
        self.values.insert(value as *const Value, pos);
    }
    
    /// Adds or overrides the position of a struct field.
    #[inline]
    pub fn set_struct_field_pos(&mut self, value: &Value, pos: Pos) {
        self.struct_fields.insert(value as *const Value, pos);
    }
}

/// A YASS value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    /// A simple atom.
    ///
    /// Example
    /// -------
    ///
    /// ```
    /// // 123
    /// let value1 = yass::Value::Atom("123".to_string());
    ///
    /// // 1.0
    /// let value2 = yass::Value::Atom("1.0".to_string());
    ///
    /// // "string"
    /// let value3 = yass::Value::Atom(r#""string""#.to_string());
    ///
    /// // "string"
    /// let value4 = yass::Value::Atom("2018-04-01".to_string());
    /// ```
    Atom(String),
    /// An array.
    ///
    /// Example
    /// -------
    /// 
    /// ```
    /// // [1 2 3]
    /// let value = yass::Value::Array(vec![
    ///     Box::new(yass::Value::Atom("1".to_string())),
    ///     Box::new(yass::Value::Atom("2".to_string())),
    ///     Box::new(yass::Value::Atom("3".to_string())),
    /// ]);
    /// ```
    Array(Vec<Box<Value>>),
    /// A tagged value.
    ///
    /// Example
    /// -------
    /// 
    /// ```
    /// // (tag)value
    /// let value = yass::Value::Tagged("tag".to_string(), Box::new(yass::Value::Atom("value".to_string())));
    /// ```
    Tagged(String, Box<Value>),
    /// A struct
    ///
    /// Example
    /// -------
    /// 
    /// ```
    /// // {
    /// //   key1 "value1"
    /// //   key2 value2
    /// //   key3 -1.0
    /// // }
    /// let value = yass::Value::Struct(vec![
    ///     yass::StructField {
    ///         key: "key1".to_string(),
    ///         value: Box::new(yass::Value::Atom(r#""value1""#.to_string())),
    ///     },
    ///     yass::StructField {
    ///         key: "key2".to_string(),
    ///         value: Box::new(yass::Value::Atom("value2".to_string())),
    ///     },
    ///     yass::StructField {
    ///         key: "key3".to_string(),
    ///         value: Box::new(yass::Value::Atom("-1.0".to_string())),
    ///     },
    /// ]);
    /// ```
    Struct(Vec<StructField>),
}

/// Struct field
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructField {
    pub key: String,
    pub value: Box<Value>,
}

/// A full YASS document, with a header.
///
/// Example
/// -------
/// 
/// ```
/// // (example)
/// // key1 "value1"
/// // key2 value2
/// // key3 -1.0
/// let document = yass::Document {
///     header: "example".to_string(),
///     root_fields: vec![
///         yass::StructField {
///             key: "key1".to_string(),
///             value: Box::new(yass::Value::Atom(r#""value1""#.to_string())),
///         },
///         yass::StructField {
///             key: "key2".to_string(),
///             value: Box::new(yass::Value::Atom("value2".to_string())),
///         },
///         yass::StructField {
///             key: "key3".to_string(),
///             value: Box::new(yass::Value::Atom("-1.0".to_string())),
///         },
///     ],
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Document {
    pub header: String,
    pub root_fields: Vec<StructField>,
}

/// Macro to define values with a lighter syntax.
///
/// Example
/// -------
/// ```
/// #[macro_use]
/// extern crate yass;
/// 
/// fn main() {
///     // 123
///     let value1 = yass::Value::Atom("123".to_string());
///     let value2 = yass_value!(("123"));
///     assert_eq!(value1, *value2);
///     
///     // [1 2 3]
///     let value1 = yass::Value::Array(vec![
///         Box::new(yass::Value::Atom("1".to_string())),
///         Box::new(yass::Value::Atom("2".to_string())),
///         Box::new(yass::Value::Atom("3".to_string())),
///     ]);
///     let value2 = yass_value!(["1", "2", "3"]);
///     assert_eq!(value1, *value2);
///     
///     // (tag)value
///     let value1 = yass::Value::Tagged("tag".to_string(), Box::new(yass::Value::Atom("value".to_string())));
///     let value2 = yass_value!((as "tag": "value"));
///     assert_eq!(value1, *value2);
///     
///     // {
///     //   key1 "value1"
///     //   key2 value2
///     //   key3 -1.0
///     // }
///     let value1 = yass::Value::Struct(vec![
///         yass::StructField {
///             key: "key1".to_string(),
///             value: Box::new(yass::Value::Atom(r#""value1""#.to_string())),
///         },
///         yass::StructField {
///             key: "key2".to_string(),
///             value: Box::new(yass::Value::Atom("value2".to_string())),
///         },
///         yass::StructField {
///             key: "key3".to_string(),
///             value: Box::new(yass::Value::Atom("-1.0".to_string())),
///         },
///     ]);
///     let value2 = yass_value!({"key1": r#""value1""#, "key2": "value2", "key3": "-1.0"});
///     assert_eq!(value1, *value2);
/// }
/// ```
#[macro_export]
macro_rules! yass_value {
    ({$($key:tt: $value:tt),*}) => {
        Box::new($crate::Value::Struct(vec![
            $($crate::StructField {
                key: $key.into(),
                value: yass_value!($value),
            }),*
        ]))
    };
    
    ([$($item:tt),*]) => {
        Box::new($crate::Value::Array(vec![$(yass_value!($item)),*]))
    };
    
    ((as $tag:tt: $value:tt)) => {
        Box::new($crate::Value::Tagged($tag.into(), yass_value!($value)))
    };
    
    ($value:tt) => {
        Box::new($crate::Value::Atom($value.into()))
    };
}

/// Macro to define documents with a lighter syntax.
///
/// Example
/// -------
/// ```
/// #[macro_use]
/// extern crate yass;
/// 
/// fn main() {
///     // (example)
///     // key1 "value1"
///     // key2 value2
///     // key3 -1.0
///     let document1 = yass::Document {
///         header: "example".to_string(),
///         root_fields: vec![
///             yass::StructField {
///                 key: "key1".to_string(),
///                 value: Box::new(yass::Value::Atom(r#""value1""#.to_string())),
///             },
///             yass::StructField {
///                 key: "key2".to_string(),
///                 value: Box::new(yass::Value::Atom("value2".to_string())),
///             },
///             yass::StructField {
///                 key: "key3".to_string(),
///                 value: Box::new(yass::Value::Atom("-1.0".to_string())),
///             },
///         ],
///     };
///     let document2 = yass_document!(
///         ("example")
///         "key1": r#""value1""#,
///         "key2": "value2",
///         "key3": "-1.0"
///     );
///     assert_eq!(document1, document2);
/// }
/// ```
#[macro_export]
macro_rules! yass_document {
    (($header:expr) $($key:tt: $value:tt),*) => {
        $crate::Document {
            header: $header.into(),
            root_fields: vec![
                $($crate::StructField {
                    key: $key.into(),
                    value: yass_value!($value),
                }),*
            ],
        }
    };
}
