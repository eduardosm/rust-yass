// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! YASS parser
//!
//! Example
//! -------
//!
//! ```
//! #[macro_use]
//! extern crate yass;
//! extern crate yass_parser;
//!
//! fn main() {
//!     // (example)
//!     // key1 "value1"
//!     // key2 value2
//!     // key3 -1.0
//!     let text = b"(example)\nkey1 \"value1\"\nkey2 value2\nkey3 -1.0";
//!     
//!     let parser_limits = yass_parser::ParserLimits::unlimited();
//!     let (parsed, pos_map) = yass_parser::parse(parser_limits, text).unwrap();
//!     
//!     let expected = yass_document!(
//!         ("example")
//!         "key1": r#""value1""#,
//!         "key2": "value2",
//!         "key3": "-1.0"
//!     );
//!     assert_eq!(parsed, expected);
//! }
//! ```

#[allow(unused_imports)]
#[macro_use]
extern crate yass;

#[cfg(test)]
mod tests;

// Token
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Atom,
}

struct Token<'a> {
    kind: TokenKind,
    data: &'a [u8],
    pos: yass::Pos,
}

// Parser
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParserError {
    IllegalChr {
        pos: yass::Pos,
        chr: u8,
    },
    IllegalChrInString {
        pos: yass::Pos,
        chr: u8,
    },
    UnfinishedString {
        pos: yass::Pos,
    },
    IllegalChrAfterAtom {
        pos: yass::Pos,
        chr: u8,
    },
    UnexpectedToken {
        pos: yass::Pos,
        token_kind: TokenKind,
    },
    ExpectedToken {
        pos: yass::Pos,
        token_kind: TokenKind,
    },
    TooDeep {
        pos: yass::Pos,
    },
    AtomTooLong {
        pos: yass::Pos,
    },
    TagTooLong {
        pos: yass::Pos,
    },
    KeyTooLong {
        pos: yass::Pos,
    },
    ArrayTooBig {
        pos: yass::Pos,
    },
    StructTooBig {
        pos: yass::Pos,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParserError::IllegalChr { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} at {}:{}", chr, pos.line + 1, pos.column + 1)
            }
            ParserError::IllegalChrInString { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} in string at {}:{}", chr, pos.line + 1, pos.column + 1)
            }
            ParserError::UnfinishedString { pos } => {
                write!(f, "Unfinished string at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::IllegalChrAfterAtom { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} after atom at {}:{}", chr, pos.line + 1, pos.column + 1)
            }
            ParserError::UnexpectedToken { pos, token_kind } => {
                write!(f, "Unexpected token {:?} at {}:{}", token_kind, pos.line + 1, pos.column + 1)
            }
            ParserError::ExpectedToken { pos, token_kind } => {
                write!(f, "Expected token {:?} at {}:{}", token_kind, pos.line + 1, pos.column + 1)
            }
            ParserError::TooDeep { pos } => {
                write!(f, "Maximum depth exceeded at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::AtomTooLong { pos } => {
                write!(f, "Atom too long at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::TagTooLong { pos } => {
                write!(f, "Tag too long at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::KeyTooLong { pos } => {
                write!(f, "Struct key too long at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::ArrayTooBig { pos } => {
                write!(f, "Array too big at {}:{}", pos.line + 1, pos.column + 1)
            }
            ParserError::StructTooBig { pos } => {
                write!(f, "Struct too big at {}:{}", pos.line + 1, pos.column + 1)
            }
        }
    }
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::IllegalChr { .. } => "Illegal character",
            ParserError::IllegalChrInString { .. } => "Illegal character in string",
            ParserError::UnfinishedString { .. } => "Unfinished string",
            ParserError::IllegalChrAfterAtom { .. } => "Illegal character after atom",
            ParserError::UnexpectedToken { .. } => "Unexpected token",
            ParserError::ExpectedToken { .. } => "Expected token",
            ParserError::TooDeep { .. } => "Maximum depth exceeded",
            ParserError::AtomTooLong { .. } => "Atom too long",
            ParserError::TagTooLong { .. } => "Tag too long",
            ParserError::KeyTooLong { .. } => "Struct key too long",
            ParserError::ArrayTooBig { .. } => "Array too big",
            ParserError::StructTooBig { .. } => "Struct too big",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ParserLimits {
    pub max_depth: usize,
    pub max_atom_length: usize,
    pub max_key_length: usize,
    pub max_tag_length: usize,
    pub max_array_size: usize,
    pub max_struct_size: usize,
}

impl ParserLimits {
    #[inline]
    pub fn unlimited() -> Self {
        Self {
            max_depth: usize::max_value(),
            max_atom_length: usize::max_value(),
            max_key_length: usize::max_value(),
            max_tag_length: usize::max_value(),
            max_array_size: usize::max_value(),
            max_struct_size: usize::max_value(),
        }
    }
}

struct Parser<'a> {
    limits: ParserLimits,
    lexer: Lexer<'a>,
    depth: usize,
    pos_map: yass::PosMap,
}

impl<'a> Parser<'a> {
    #[inline]
    fn parse(limits: ParserLimits, data: &'a [u8]) -> Result<(yass::Document, yass::PosMap), ParserError> {
        let mut parser = Self {
            limits: limits,
            lexer: Lexer::new(data),
            depth: 0,
            pos_map: yass::PosMap::new(),
        };
        
        let token = parser.lexer.get_token()?;
        if token.kind != TokenKind::LeftParen {
            return Err(ParserError::ExpectedToken { pos: token.pos, token_kind: TokenKind::LeftParen });
        }
        
        let token = parser.lexer.get_token()?;
        if token.kind != TokenKind::Atom {
            return Err(ParserError::ExpectedToken { pos: token.pos, token_kind: TokenKind::Atom });
        }
        let header = std::str::from_utf8(token.data).unwrap().to_string();
        
        let token = parser.lexer.get_token()?;
        if token.kind != TokenKind::RightParen {
            return Err(ParserError::ExpectedToken { pos: token.pos, token_kind: TokenKind::RightParen });
        }
        
        let mut root_fields = Vec::new();
        loop {
            let token = parser.lexer.get_token()?;
            if token.kind == TokenKind::Eof {
                break;
            }
            
            if token.kind != TokenKind::Atom {
                return Err(ParserError::UnexpectedToken { pos: token.pos, token_kind: token.kind });
            }
            
            if token.data.len() > parser.limits.max_key_length {
                return Err(ParserError::KeyTooLong { pos: token.pos });
            }
            
            let field_pos = token.pos;
            let key = String::from_utf8(token.data.to_vec()).unwrap();
            let value_1st_token = parser.lexer.get_token()?;
            let value = parser.parse_value(value_1st_token)?;
            parser.pos_map.set_struct_field_pos(&value, field_pos);
            
            root_fields.push(yass::StructField { key: key, value: value });
        }
        
        Ok((yass::Document { header: header, root_fields: root_fields }, parser.pos_map))
    }
    
    fn parse_value(&mut self, token: Token<'a>) -> Result<Box<yass::Value>, ParserError> {
        match token.kind {
            TokenKind::Atom => {
                if token.data.len() > self.limits.max_atom_length {
                    return Err(ParserError::AtomTooLong { pos: token.pos });
                }
                
                let value = Box::new(yass::Value::Atom(std::str::from_utf8(token.data).unwrap().to_string()));
                self.pos_map.set_value_pos(&value, token.pos);
                Ok(value)
            }
            TokenKind::LeftBracket => {
                if self.depth == self.limits.max_depth {
                    return Err(ParserError::TooDeep { pos: token.pos });
                }
                
                let begin_pos = token.pos;
                let mut items = Vec::new();
                self.depth += 1;
                loop {
                    let token = self.lexer.get_token()?;
                    if token.kind == TokenKind::RightBracket {
                        break;
                    }
                    
                    if items.len() == self.limits.max_array_size {
                        return Err(ParserError::ArrayTooBig { pos: token.pos });
                    }
                    
                    let value = self.parse_value(token)?;
                    items.push(value);
                }
                self.depth -= 1;
                
                let value = Box::new(yass::Value::Array(items));
                self.pos_map.set_value_pos(&value, begin_pos);
                Ok(value)
            }
            TokenKind::LeftBrace => {
                if self.depth == self.limits.max_depth {
                    return Err(ParserError::TooDeep { pos: token.pos });
                }
                
                let begin_pos = token.pos;
                let mut fields = Vec::new();
                self.depth += 1;
                loop {
                    let token = self.lexer.get_token()?;
                    if token.kind == TokenKind::RightBrace {
                        break;
                    }
                    
                    if token.kind != TokenKind::Atom {
                        return Err(ParserError::UnexpectedToken { pos: token.pos, token_kind: token.kind });
                    }
                    
                    if fields.len() == self.limits.max_struct_size {
                        return Err(ParserError::StructTooBig { pos: token.pos });
                    }
                    
                    if token.data.len() > self.limits.max_key_length {
                        return Err(ParserError::KeyTooLong { pos: token.pos });
                    }
                    
                    let field_pos = token.pos;
                    let key = String::from_utf8(token.data.to_vec()).unwrap();
                    let value_1st_token = self.lexer.get_token()?;
                    let value = self.parse_value(value_1st_token)?;
                    self.pos_map.set_struct_field_pos(&value, field_pos);
                    
                    fields.push(yass::StructField { key: key, value: value });
                }
                self.depth -= 1;
                
                let value = Box::new(yass::Value::Struct(fields));
                self.pos_map.set_value_pos(&value, begin_pos);
                Ok(value)
            }
            TokenKind::LeftParen => {
                if self.depth == self.limits.max_depth {
                    return Err(ParserError::TooDeep { pos: token.pos });
                }
                
                let begin_pos = token.pos;
                let token = self.lexer.get_token()?;
                if token.kind != TokenKind::Atom {
                    return Err(ParserError::ExpectedToken { pos: token.pos, token_kind: TokenKind::Atom });
                }
                if token.data.len() > self.limits.max_tag_length {
                    return Err(ParserError::TagTooLong { pos: token.pos });
                }
                let tag = std::str::from_utf8(token.data).unwrap().to_string();
                
                let token = self.lexer.get_token()?;
                if token.kind != TokenKind::RightParen {
                    return Err(ParserError::ExpectedToken { pos: token.pos, token_kind: TokenKind::RightParen });
                }
                
                let value_1st_token = self.lexer.get_token()?;
                self.depth += 1;
                let sub_value = self.parse_value(value_1st_token)?;
                self.depth -= 1;
                
                let value = Box::new(yass::Value::Tagged(tag, sub_value));
                self.pos_map.set_value_pos(&value, begin_pos);
                Ok(value)
            }
            _ => {
                Err(ParserError::UnexpectedToken { pos: token.pos, token_kind: token.kind })
            }
        }
    }
}

#[inline]
pub fn parse(limits: ParserLimits, data: &[u8]) -> Result<(yass::Document, yass::PosMap), ParserError> {
    Parser::parse(limits, data)
}

#[derive(Debug)]
pub enum ParseStreamError {
    ReadError(std::io::Error),
    ParserError(ParserError),
}

impl std::fmt::Display for ParseStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseStreamError::ReadError(ref io_error) => {
                write!(f, "Read error: {}", io_error)
            }
            ParseStreamError::ParserError(ref parser_error) => {
                write!(f, "Parse error: {}", parser_error)
            }
        }
    }
}

impl std::error::Error for ParseStreamError {
    fn description(&self) -> &str {
        match *self {
            ParseStreamError::ReadError(_) => "Read error",
            ParseStreamError::ParserError(_) => "Parse error",
        }
    }
    
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ParseStreamError::ReadError(ref io_error) => Some(io_error),
            ParseStreamError::ParserError(ref parser_error) => Some(parser_error),
        }
    }
}

#[inline]
pub fn parse_stream(limits: ParserLimits, stream: &mut std::io::Read)
    -> Result<(Vec<u8>, yass::Document, yass::PosMap), ParseStreamError>
{
    let mut data = Vec::new();
    stream.read_to_end(&mut data).map_err(|e| ParseStreamError::ReadError(e))?;
    parse(limits, &data).map_err(|e| ParseStreamError::ParserError(e)).map(|(doc, pos_map)| (data, doc, pos_map))
}

#[derive(Debug)]
pub enum ParseFileError {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
    ParserError(ParserError),
}

impl std::fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseFileError::OpenError(ref io_error) => {
                write!(f, "Open error: {}", io_error)
            }
            ParseFileError::ReadError(ref io_error) => {
                write!(f, "Read error: {}", io_error)
            }
            ParseFileError::ParserError(ref parser_error) => {
                write!(f, "Parse error: {}", parser_error)
            }
        }
    }
}

impl std::error::Error for ParseFileError {
    fn description(&self) -> &str {
        match *self {
            ParseFileError::OpenError(_) => "Open error",
            ParseFileError::ReadError(_) => "Read error",
            ParseFileError::ParserError(_) => "Parse error",
        }
    }
    
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ParseFileError::OpenError(ref io_error) => Some(io_error),
            ParseFileError::ReadError(ref io_error) => Some(io_error),
            ParseFileError::ParserError(ref parser_error) => Some(parser_error),
        }
    }
}

#[inline]
pub fn parse_file<P: std::convert::AsRef<std::path::Path>>(limits: ParserLimits, file_path: &P)
    -> Result<(Vec<u8>, yass::Document, yass::PosMap), ParseFileError>
{
    let mut file = std::fs::OpenOptions::new().read(true).open(file_path.as_ref()).map_err(|e| ParseFileError::OpenError(e))?;
    let mut data = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut data).map_err(|e| ParseFileError::ReadError(e))?;
    std::mem::drop(file);
    parse(limits, &data).map_err(|e| ParseFileError::ParserError(e)).map(|(doc, pos_map)| (data, doc, pos_map))
}

// Lexer
struct Lexer<'a> {
    data: &'a [u8],
    i: usize,
    pos: yass::Pos,
}

impl<'a> Lexer<'a> {
    #[inline]
    fn new(data: &'a [u8]) -> Self {
        Self {
            data: data,
            i: 0,
            pos: yass::Pos::new(0, 0),
        }
    }
    
    #[inline]
    fn make_token(&mut self, kind: TokenKind, len: u32) -> Token<'a> {
        let token = Token {
            kind: kind,
            data: &self.data[self.i .. (self.i + len as usize)],
            pos: self.pos,
        };
        
        self.i += len as usize;
        self.pos.column += len;
        
        token
    }
    
    fn skip_comment(&mut self) {
        while self.i < self.data.len() {
            match self.data[self.i] {
                b'\n' | b'\r' => break,
                _ => {
                    self.i += 1;
                    self.pos.column += 1;
                }
            }
        }
    }
    
    fn skip_spaces_and_comments(&mut self) {
        let mut last_was_cr = false;
        
        while self.i < self.data.len() {
            match self.data[self.i] {
                b' ' | b'\t' => {
                    self.i += 1;
                    self.pos.column += 1;
                    last_was_cr = false;
                }
                b'\n' => {
                    self.i += 1;
                    if !last_was_cr {
                        self.pos.line += 1;
                        self.pos.column = 0;
                    } else {
                        last_was_cr = false;
                    }
                }
                b'\r' => {
                    self.i += 1;
                    self.pos.line += 1;
                    self.pos.column = 0;
                    last_was_cr = true;
                }
                b'\\' => {
                    self.i += 1;
                    self.pos.column += 1;
                    self.skip_comment();
                }
                _ => break,
            }
        }
    }
    
    fn get_token(&mut self) -> Result<Token<'a>, ParserError> {
        self.skip_spaces_and_comments();
        
        if self.i == self.data.len() {
            return Ok(self.make_token(TokenKind::Eof, 0));
        }
        
        match self.data[self.i] {
            b'(' => Ok(self.make_token(TokenKind::LeftParen, 1)),
            b')' => Ok(self.make_token(TokenKind::RightParen, 1)),
            b'[' => Ok(self.make_token(TokenKind::LeftBracket, 1)),
            b']' => Ok(self.make_token(TokenKind::RightBracket, 1)),
            b'{' => Ok(self.make_token(TokenKind::LeftBrace, 1)),
            b'}' => Ok(self.make_token(TokenKind::RightBrace, 1)),
            chr if Self::is_atom_chr(chr) || chr == b'"' => {
                let left = self.data.len() - self.i;
                let mut len: u32 = 0;
                while (len as usize) != left {
                    let chr = self.data[self.i + len as usize];
                    if chr == b'"' {
                        len += 1;
                        let mut last_was_backslash = false;
                        loop {
                            if (len as usize) == left {
                                return Err(ParserError::UnfinishedString { pos: self.pos });
                            }
                            
                            let chr = self.data[self.i + len as usize];
                            
                            if !Self::is_string_chr(chr) {
                                let err_pos = yass::Pos::new(self.pos.line, self.pos.column + len);
                                return Err(ParserError::IllegalChrInString { pos: err_pos, chr: chr, });
                            }
                            
                            if !last_was_backslash {
                                if chr == b'\\' {
                                    last_was_backslash = true;
                                } else if chr == b'"' {
                                    len += 1;
                                    break;
                                }
                            } else {
                                last_was_backslash = false;
                            }
                            
                            len += 1;
                        }
                    } else if Self::is_atom_chr(chr) {
                        len += 1;
                    } else {
                        break;
                    }
                }
                Ok(self.make_token(TokenKind::Atom, len))
            }
            chr => Err(ParserError::IllegalChr { pos: self.pos, chr: chr })
        }
    }
    
    fn is_atom_chr(chr: u8) -> bool {
        match chr {
            b'_' | b'.' | b',' | b':' | b';' | b'+' | b'-' | b'*' | b'/' => true,
            b'|' | b'$' | b'#' | b'@' | b'%' | b'=' | b'<' | b'>' => true,
            b'a' ... b'z' | b'A' ... b'Z' | b'0' ... b'9' | b'?' | b'!' => true,
            _ => false,
        }
    }
    
    fn is_string_chr(chr: u8) -> bool {
        match chr {
            0x00 ... 0x1F | 0x7F ... 0xFF => false,
            _ => true,
        }
    }
}
