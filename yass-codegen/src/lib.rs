// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod schema;

use std::collections::HashMap;

struct CodeGen<'a> {
    type_def_name_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodeGen<'a> {
    fn gen_code_for_schema(schema: &'a schema::Schema<'a>, output: &mut String) {
        let mut code_gen = Self {
            type_def_name_map: HashMap::new(),
        };
        
        let mut code_writer = CodeWriter::new();
        
        for type_def in schema.type_defs.iter() {
            let (yass_name, code_name) = match *type_def {
                schema::TypeDef::Struct(ref struct_def) => {
                    (struct_def.yass_name, struct_def.code_name)
                }
                schema::TypeDef::TaggedUnion(ref tagged_union_def) => {
                    (tagged_union_def.yass_name, tagged_union_def.code_name)
                }
                schema::TypeDef::Enum(ref enum_def) => {
                    (enum_def.yass_name, enum_def.code_name)
                }
            };
            
            let existed = code_gen.type_def_name_map.insert(yass_name, code_name).is_some();
            assert!(!existed, "Type definition {:?} already existed.", yass_name);
        }
        
        for type_def in schema.type_defs.iter() {
            match *type_def {
                schema::TypeDef::Struct(ref struct_def) => {
                    if struct_def.yass_name == schema.root_type {
                        code_gen.gen_code_for_struct_def(struct_def, Some(schema.header), &mut code_writer);
                    } else {
                        code_gen.gen_code_for_struct_def(struct_def, None, &mut code_writer);
                    }
                }
                schema::TypeDef::TaggedUnion(ref tagged_union_def) => {
                    code_gen.gen_code_for_tagged_union_def(tagged_union_def, &mut code_writer);
                }
                schema::TypeDef::Enum(ref enum_def) => {
                    code_gen.gen_code_for_enum_def(enum_def, &mut code_writer);
                }
            }
        }
        
        code_writer.write_to(output);
    }
    
    fn gen_code_for_struct_def(&self, struct_def: &schema::StructDef, root_header: Option<&str>, code_writer: &mut CodeWriter) {
        code_writer.add_line("#[derive(Clone, Debug, PartialEq)]".to_string());
        code_writer.add_line(format!("pub struct {} {{", struct_def.code_name));
        code_writer.with_indent(|code_writer| {
            for field in struct_def.fields {
                let code_field_type = self.type_to_rs_type(&field.type_);
                match field.mode {
                    schema::StructFieldMode::SingleOptional => {
                        code_writer.add_line(format!("pub {}: Option<{}>,", field.code_name, code_field_type));
                    }
                    schema::StructFieldMode::SingleRequired => {
                        code_writer.add_line(format!("pub {}: {},", field.code_name, code_field_type));
                    }
                    schema::StructFieldMode::MultipleOptional => {
                        code_writer.add_line(format!("pub {}: Vec<{}>,", field.code_name, code_field_type));
                    }
                    schema::StructFieldMode::MultipleRequired => {
                        code_writer.add_line(format!("pub {}: Vec<{}>,", field.code_name, code_field_type));
                    }
                }
            }
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        code_writer.add_line("#[allow(unused_parens)]".to_string());
        code_writer.add_line(format!("impl {} {{", struct_def.code_name));
        code_writer.inc_indent(1);
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn to_yass_value(&self) -> yass::Value {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("yass::Value::Struct(self.to_yass_struct())".to_string());
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        if let Some(root_header) = root_header {
            code_writer.add_line("#[allow(dead_code)]".to_string());
            code_writer.add_line("pub fn to_yass_document(&self) -> yass::Document {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("yass::Document {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line(format!("header: {:?}.to_string(),", root_header));
                    code_writer.add_line("root_fields: self.to_yass_struct(),".to_string());
                });
                code_writer.add_line("}".to_string());
            });
            code_writer.add_line("}".to_string());
            code_writer.add_empty_line();
        }
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn to_yass_struct(&self) -> Vec<yass::StructField> {".to_string());
        code_writer.with_indent(|code_writer| {
            if struct_def.fields.len() != 0 {
                code_writer.add_line("let mut fields = Vec::new();".to_string());
            } else {
                code_writer.add_line("let fields = Vec::new();".to_string());
            }
            
            for field in struct_def.fields {
                match field.mode {
                    schema::StructFieldMode::SingleOptional => {
                        code_writer.add_line(format!("if let Some(ref value) = self.{} {{", field.code_name));
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("fields.push(yass::StructField {".to_string());
                            code_writer.with_indent(|code_writer| {
                                code_writer.add_line(format!("key: {:?}.to_string(),", field.yass_name));
                                self.gen_to_yass_value(&field.type_, "value: Box::new(", "),", "(*value)", code_writer);
                            });
                            code_writer.add_line("});".to_string());
                        });
                        code_writer.add_line("}".to_string());
                    }
                    schema::StructFieldMode::SingleRequired => {
                        code_writer.add_line("fields.push(yass::StructField {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line(format!("key: {:?}.to_string(),", field.code_name));
                            self.gen_to_yass_value(&field.type_, "value: Box::new(", "),", format!("self.{}", field.code_name).as_str(), code_writer);
                        });
                        code_writer.add_line("});".to_string());
                    }
                    schema::StructFieldMode::MultipleOptional |
                    schema::StructFieldMode::MultipleRequired => {
                        code_writer.add_line(format!("for value in self.{}.iter() {{", field.code_name));
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("fields.push(yass::StructField {".to_string());
                            code_writer.with_indent(|code_writer| {
                                code_writer.add_line(format!("key: {:?}.to_string(),", field.code_name));
                                self.gen_to_yass_value(&field.type_, "value: Box::new(", "),", "(*value)", code_writer);
                            });
                            code_writer.add_line("});".to_string());
                        });
                        code_writer.add_line("}".to_string());
                    }
                }
            }
            code_writer.add_line("fields".to_string());
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn from_yass_value(value: &yass::Value, pos_map: &yass::PosMap) -> Result<Self, yass_schema_error::Error> {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("if let yass::Value::Struct(ref fields) = *value {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("Self::from_yass_struct(fields.as_slice(), pos_map.get_value_pos(value), pos_map)".to_string());
            });
            
            code_writer.add_line("} else {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("Err(yass_schema_error::Error::InvalidValueTypeForStruct {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line(format!("struct_name: {:?}.to_string(),", struct_def.yass_name));
                    code_writer.add_line("value_pos: pos_map.get_value_pos(value),".to_string());
                });
                code_writer.add_line("})".to_string());
            });
            code_writer.add_line("}".to_string());
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        if let Some(root_header) = root_header {
            code_writer.add_line("#[allow(dead_code)]".to_string());
            code_writer.add_line("pub fn from_yass_document(document: &yass::Document, pos_map: &yass::PosMap) -> Result<Self, yass_schema_error::Error> {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line(format!("if document.header != {:?} {{", root_header));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidDocumentHeader {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("header: document.header.clone(),".to_string());
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line("}".to_string());
                code_writer.add_empty_line();
                code_writer.add_line("Self::from_yass_struct(document.root_fields.as_slice(), Some(yass::Pos::new(0, 0)), pos_map)".to_string());
            });
            code_writer.add_line("}".to_string());
        }
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn from_yass_struct(fields: &[yass::StructField], _pos: Option<yass::Pos>, pos_map: &yass::PosMap) -> Result<Self, yass_schema_error::Error> {".to_string());
        code_writer.with_indent(|code_writer| {
            for field in struct_def.fields {
                let code_field_type = self.type_to_rs_type(&field.type_);
                match field.mode {
                    schema::StructFieldMode::SingleOptional |
                    schema::StructFieldMode::SingleRequired => {
                        code_writer.add_line(format!("let mut field_{}: Option<{}> = None;", field.code_name, code_field_type));
                    }
                    schema::StructFieldMode::MultipleOptional |
                    schema::StructFieldMode::MultipleRequired => {
                        code_writer.add_line(format!("let mut fields_{}: Vec<{}> = Vec::new();", field.code_name, code_field_type));
                    }
                }
            }
            code_writer.add_empty_line();
            
            code_writer.add_line("for field in fields {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("match field.key.as_str() {".to_string());
                code_writer.with_indent(|code_writer| {
                    for field in struct_def.fields {
                        code_writer.add_line(format!("{:?} => {{", field.yass_name));
                        code_writer.with_indent(|code_writer| {
                            match field.mode {
                                schema::StructFieldMode::SingleOptional |
                                schema::StructFieldMode::SingleRequired => {
                                    code_writer.add_line(format!("if field_{}.is_some() {{", field.code_name));
                                    code_writer.with_indent(|code_writer| {
                                        code_writer.add_line("return Err(yass_schema_error::Error::RepeatedStructField {".to_string());
                                        code_writer.with_indent(|code_writer| {
                                            code_writer.add_line(format!("struct_name: {:?}.to_string(),", struct_def.yass_name));
                                            code_writer.add_line(format!("field_name: {:?}.to_string(),", field.yass_name));
                                            code_writer.add_line("field_pos: pos_map.get_struct_field_pos(&field.value),".to_string());
                                        });
                                        code_writer.add_line("});".to_string());
                                    });
                                    code_writer.add_line("} else {".to_string());
                                    code_writer.with_indent(|code_writer| {
                                        let prefix = format!("field_{} = Some(", field.code_name);
                                        let suffix = ");";
                                        self.gen_from_yass_value(&field.type_, prefix.as_str(), suffix, "(*field.value)", code_writer);
                                    });
                                    code_writer.add_line("}".to_string());
                                }
                                schema::StructFieldMode::MultipleOptional |
                                schema::StructFieldMode::MultipleRequired => {
                                    let prefix = format!("fields_{}.push(", field.code_name);
                                    let suffix = ")";
                                    self.gen_from_yass_value(&field.type_, prefix.as_str(), suffix, "(*field.value)", code_writer);
                                }
                            }
                        });
                        code_writer.add_line("}".to_string());
                    }
                    code_writer.add_line("_ => {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::UnknownStructField {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line(format!("struct_name: {:?}.to_string(),", struct_def.yass_name));
                            code_writer.add_line("field_name: field.key.clone(),".to_string());
                            code_writer.add_line("field_pos: pos_map.get_struct_field_pos(&field.value),".to_string());
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("}".to_string());
            });
            code_writer.add_line("}".to_string());
            code_writer.add_empty_line();
            
            for field in struct_def.fields {
                match field.mode {
                    schema::StructFieldMode::SingleOptional => {}
                    schema::StructFieldMode::SingleRequired => {
                        code_writer.add_line(format!("let field_{} = if let Some(field) = field_{} {{", field.code_name, field.code_name));
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("field".to_string());
                        });
                        code_writer.add_line("} else {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("return Err(yass_schema_error::Error::MissingStructField {".to_string());
                            code_writer.with_indent(|code_writer| {
                                code_writer.add_line(format!("struct_name: {:?}.to_string(),", struct_def.yass_name));
                                code_writer.add_line(format!("field_name: {:?}.to_string(),", field.yass_name));
                                code_writer.add_line("struct_pos: _pos,".to_string());
                            });
                            code_writer.add_line("});".to_string());
                        });
                        code_writer.add_line("};".to_string());
                    }
                    schema::StructFieldMode::MultipleOptional => {}
                    schema::StructFieldMode::MultipleRequired => {
                        code_writer.add_line(format!("if fields_{}.is_empty() {{", field.code_name));
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("return Err(yass_schema_error::Error::MissingStructField {".to_string());
                            code_writer.with_indent(|code_writer| {
                                code_writer.add_line(format!("struct_name: {:?}.to_string(),", struct_def.yass_name));
                                code_writer.add_line(format!("field_name: {:?}.to_string(),", field.yass_name));
                                code_writer.add_line("struct_pos: _pos,".to_string());
                            });
                            code_writer.add_line("});".to_string());
                        });
                        code_writer.add_line("}".to_string());
                    }
                    
                }
                code_writer.add_empty_line();
            }
            
            code_writer.add_line("Ok(Self {".to_string());
            code_writer.with_indent(|code_writer| {
                for field in struct_def.fields {
                    match field.mode {
                        schema::StructFieldMode::SingleOptional |
                        schema::StructFieldMode::SingleRequired => {
                            code_writer.add_line(format!("{}: field_{},", field.code_name, field.code_name));
                        }
                        schema::StructFieldMode::MultipleOptional |
                        schema::StructFieldMode::MultipleRequired => {
                            code_writer.add_line(format!("{}: fields_{},", field.code_name, field.code_name));
                        }
                    }
                }
            });
            code_writer.add_line("})".to_string());
        });
        code_writer.add_line("}".to_string());
        
        code_writer.dec_indent(1);
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
    }
    
    fn gen_code_for_tagged_union_def(&self, tagged_union_def: &schema::TaggedUnionDef, code_writer: &mut CodeWriter) {
        code_writer.add_line("#[derive(Clone, Debug, PartialEq)]".to_string());
        code_writer.add_line(format!("pub enum {} {{", tagged_union_def.code_name));
        code_writer.with_indent(|code_writer| {
            for variant in tagged_union_def.variants {
                let code_variant_type = self.type_to_rs_type(&variant.type_);
                code_writer.add_line(format!("{}({}),", variant.code_name, code_variant_type));
            }
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        code_writer.add_line("#[allow(unused_parens)]".to_string());
        code_writer.add_line(format!("impl {} {{", tagged_union_def.code_name));
        code_writer.inc_indent(1);
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn to_yass_value(&self) -> yass::Value {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("match *self {".to_string());
            code_writer.with_indent(|code_writer| {
                for variant in tagged_union_def.variants {
                    code_writer.add_line(format!("{}::{}(ref value) => {{", tagged_union_def.code_name, variant.code_name));
                    code_writer.with_indent(|code_writer| {
                        let prefix = format!("yass::Value::Tagged({:?}.to_string(), Box::new(", variant.yass_name);
                        self.gen_to_yass_value(&variant.type_, prefix.as_str(), "))", "(*value)", code_writer);
                    });
                    code_writer.add_line("}".to_string());
                }
            });
            code_writer.add_line("}".to_string());
        });
        code_writer.add_line("}".to_string());
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn from_yass_value(value: &yass::Value, pos_map: &yass::PosMap) -> Result<Self, yass_schema_error::Error> {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("if let yass::Value::Tagged(ref variant, ref vvalue) = *value {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("match variant.as_str() {".to_string());
                code_writer.with_indent(|code_writer| {
                    for variant in tagged_union_def.variants {
                        code_writer.add_line(format!("{:?} => {{", variant.yass_name));
                        code_writer.with_indent(|code_writer| {
                            let prefix = format!("Ok({}::{}(", tagged_union_def.code_name, variant.code_name);
                            self.gen_from_yass_value(&variant.type_, prefix.as_str(), "))", "(**vvalue)", code_writer);
                        });
                        code_writer.add_line("}".to_string());
                    }
                    code_writer.add_line("_ => {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::UnknownTaggedUnionVariant {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line(format!("tagged_union_name: {:?}.to_string(),", tagged_union_def.yass_name));
                            code_writer.add_line("variant_name: variant.clone(),".to_string());
                            code_writer.add_line("value_pos: pos_map.get_struct_field_pos(value),".to_string());
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("}".to_string());
            });
            code_writer.add_line("} else {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("Err(yass_schema_error::Error::InvalidValueTypeForTaggedUnion {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line(format!("tagged_union_name: {:?}.to_string(),", tagged_union_def.yass_name));
                    code_writer.add_line("value_pos: pos_map.get_value_pos(value),".to_string());
                });
                code_writer.add_line("})".to_string());
            });
            code_writer.add_line("}".to_string());
        });
        code_writer.add_line("}".to_string());
        
        code_writer.dec_indent(1);
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
    }
    
    fn gen_code_for_enum_def(&self, enum_def: &schema::EnumDef, code_writer: &mut CodeWriter) {
        code_writer.add_line("#[derive(Clone, Debug, PartialEq)]".to_string());
        code_writer.add_line(format!("pub enum {} {{", enum_def.code_name));
        code_writer.with_indent(|code_writer| {
            for value in enum_def.values {
                code_writer.add_line(format!("{},", value.code_name));
            }
            if let Some(unknown_value_name) = enum_def.unknown_value_name {
                code_writer.add_line(format!("{}(String),", unknown_value_name));
            }
        });
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
        
        code_writer.add_line("#[allow(unused_parens)]".to_string());
        code_writer.add_line(format!("impl {} {{", enum_def.code_name));
        code_writer.inc_indent(1);
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn to_yass_value(&self) -> yass::Value {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("let atom = match *self {".to_string());
            code_writer.with_indent(|code_writer| {
                for value in enum_def.values {
                    code_writer.add_line(format!("{}::{} => {:?},", enum_def.code_name, value.code_name, value.yass_name));
                }
                if let Some(unknown_value_name) = enum_def.unknown_value_name {
                    code_writer.add_line(format!("{}::{}(ref value) => value.as_str(),", enum_def.code_name, unknown_value_name));
                }
            });
            code_writer.add_line("};".to_string());
            code_writer.add_line("yass::Value::Atom(atom.to_string())".to_string());
        });
        code_writer.add_line("}".to_string());
        
        code_writer.add_line("#[allow(dead_code)]".to_string());
        code_writer.add_line("pub fn from_yass_value(value: &yass::Value, pos_map: &yass::PosMap) -> Result<Self, yass_schema_error::Error> {".to_string());
        code_writer.with_indent(|code_writer| {
            code_writer.add_line("if let yass::Value::Atom(ref atom) = *value {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("match atom.as_str() {".to_string());
                code_writer.with_indent(|code_writer| {
                    for value in enum_def.values {
                        code_writer.add_line(format!("{:?} => Ok({}::{}),", value.yass_name, enum_def.code_name, value.code_name));
                    }
                    if let Some(unknown_value_name) = enum_def.unknown_value_name {
                        code_writer.add_line(format!("_ => Ok({}::{}(atom.clone())),", enum_def.code_name, unknown_value_name));
                    } else {
                        code_writer.add_line("_ => {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("return Err(yass_schema_error::Error::UnknownEnumValue {".to_string());
                            code_writer.with_indent(|code_writer| {
                                code_writer.add_line(format!("enum_name: {:?}.to_string(),", enum_def.yass_name));
                                code_writer.add_line("value_name: atom.clone(),".to_string());
                                code_writer.add_line("value_pos: pos_map.get_struct_field_pos(value),".to_string());
                            });
                            code_writer.add_line("});".to_string());
                        });
                        code_writer.add_line("}".to_string());
                    }
                });
                code_writer.add_line("}".to_string());
            });
            code_writer.add_line("} else {".to_string());
            code_writer.with_indent(|code_writer| {
                code_writer.add_line("Err(yass_schema_error::Error::InvalidValueTypeForEnum {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line(format!("enum_name: {:?}.to_string(),", enum_def.yass_name));
                    code_writer.add_line("value_pos: pos_map.get_value_pos(value),".to_string());
                });
                code_writer.add_line("})".to_string());
            });
            code_writer.add_line("}".to_string());
        });
        code_writer.add_line("}".to_string());
        
        code_writer.dec_indent(1);
        code_writer.add_line("}".to_string());
        code_writer.add_empty_line();
    }
    
    fn type_to_rs_type(&self, type_: &schema::Type) -> String {
        match *type_ {
            schema::Type::RawAtom => "String".to_string(),
            schema::Type::Bool => "bool".to_string(),
            schema::Type::Int32 => "i32".to_string(),
            schema::Type::UInt32 => "u32".to_string(),
            schema::Type::Int64 => "i64".to_string(),
            schema::Type::UInt64 => "u64".to_string(),
            schema::Type::Float => "f64".to_string(),
            schema::Type::ByteString => "Vec<u8>".to_string(),
            schema::Type::Utf8String => "String".to_string(),
            schema::Type::AsciiString => "String".to_string(),
            schema::Type::Array(ref item_type) => format!("Vec<{}>", self.type_to_rs_type(item_type)),
            schema::Type::Tuple(ref item_types) => {
                let mut r = String::new();
                r.push('(');
                for (i, item_type) in item_types.iter().enumerate() {
                    if i != 0 {
                        r.push_str(", ");
                    }
                    r.push_str(&self.type_to_rs_type(item_type));
                }
                if item_types.len() == 1 {
                    r.push(',');
                }
                r.push(')');
                r
            }
            schema::Type::Dictionary(item_type) => format!("Vec<(String, {})>", self.type_to_rs_type(item_type)),
            schema::Type::Defined(name) => self.type_def_name_map.get(name).unwrap().to_string(),
            schema::Type::Boxed(item_type) => format!("Box<{}>", self.type_to_rs_type(item_type)),
        }
    }
    
    fn gen_to_yass_value(&self, type_: &schema::Type, prefix: &str, suffix: &str, value: &str, code_writer: &mut CodeWriter) {
        match *type_ {
            schema::Type::RawAtom => {
                code_writer.add_line(format!("{}yass::Value::Atom({}.to_string()){}", prefix, value, suffix))
            }
            schema::Type::Bool => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_bool({}).to_string()){}", prefix, value, suffix))
            }
            schema::Type::Int32 => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_i32_as_string({})){}", prefix, value, suffix))
            }
            schema::Type::UInt32 => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_u32_as_string({})){}", prefix, value, suffix))
            }
            schema::Type::Int64 => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_i64_as_string({})){}", prefix, value, suffix))
            }
            schema::Type::UInt64 => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_u64_as_string({})){}", prefix, value, suffix))
            }
            schema::Type::Float => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_f64_as_string({})){}", prefix, value, suffix))
            }
            schema::Type::ByteString => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_byte_string_as_string({}.as_slice())){}", prefix, value, suffix))
            }
            schema::Type::Utf8String => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_utf8_string_as_string({}.as_str())){}", prefix, value, suffix))
            }
            schema::Type::AsciiString => {
                code_writer.add_line(format!("{}yass::Value::Atom(yass_aux::serialize_ascii_string_as_string({}.as_str())){}", prefix, value, suffix))
            }
            schema::Type::Array(ref item_type) => {
                let prefix = format!("{}yass::Value::Array({}.iter().map(|item| Box::new(", prefix, value);
                let suffix = format!(")).collect()){}", suffix);
                self.gen_to_yass_value(item_type, prefix.as_str(), suffix.as_str(), "(**item)", code_writer);
            }
            schema::Type::Tuple(ref item_types) => {
                code_writer.add_line(format!("{}yass::Value::Array(vec![", prefix));
                code_writer.with_indent(|code_writer| {
                    for (i, item_type) in item_types.iter().enumerate() {
                        self.gen_to_yass_value(item_type, "Box::new(", "),", format!("{}.{}", value, i).as_str(), code_writer);
                    }
                });
                code_writer.add_line(format!("]){}", suffix));
            }
            schema::Type::Dictionary(ref item_type) => {
                let prefix = format!("{}yass::Value::Struct({}.iter().map(|&(ref key, ref value)| yass::StructField {{ key: key.clone(), value: Box::new(", prefix, value);
                let suffix = format!(")}}).collect()){}", suffix);
                self.gen_to_yass_value(item_type, prefix.as_str(), suffix.as_str(), "(*value)", code_writer);
            }
            schema::Type::Defined(_) => {
                code_writer.add_line(format!("{}{}.to_yass_value(){}", prefix, value, suffix));
            }
            schema::Type::Boxed(ref item_type) => {
                let value = format!("(*{})", value);
                self.gen_to_yass_value(item_type, prefix, suffix, value.as_str(), code_writer);
            }
        }
    }
    
    fn gen_from_yass_value(&self, type_: &schema::Type, prefix: &str, suffix: &str, value: &str, code_writer: &mut CodeWriter) {
        match *type_ {
            schema::Type::RawAtom => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("atom.clone()".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForRawAtom {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Bool => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(bool_value) = yass_aux::parse_bool(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("bool_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidBoolValue {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForBool {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Int32 => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(i32_value) = yass_aux::parse_i32(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("i32_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidInt32Value {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForInt32 {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::UInt32 => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(u32_value) = yass_aux::parse_u32(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("u32_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidUInt32Value {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForUInt32 {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Int64 => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(i64_value) = yass_aux::parse_i64(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("i64_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidInt64Value {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForInt64 {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::UInt64 => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(u64_value) = yass_aux::parse_u64(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("u64_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidUInt64Value {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForUInt64 {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Float => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(f64_value) = yass_aux::parse_f64(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("f64_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidFloatValue {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForFloat {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::ByteString => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(byte_string_value) = yass_aux::parse_byte_string(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("byte_string_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidByteStringValue {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForByteString {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Utf8String => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(utf8_string_value) = yass_aux::parse_utf8_string(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("utf8_string_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidUtf8StringValue {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForUtf8String {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::AsciiString => {
                code_writer.add_line(format!("{}if let yass::Value::Atom(ref atom) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("if let Some(ascii_string_value) = yass_aux::parse_ascii_string(atom.as_str()) {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("ascii_string_value".to_string());
                    });
                    code_writer.add_line("} else {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidAsciiStringValue {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("value: atom.clone(),".to_string());
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForAsciiString {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Array(ref item_type) => {
                code_writer.add_line(format!("{}if let yass::Value::Array(ref items) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("let mut vec = Vec::with_capacity(items.len());".to_string());
                    code_writer.add_line("for item in items {".to_string());
                    code_writer.with_indent(|code_writer| {
                        self.gen_from_yass_value(item_type, "vec.push(", ");", "(**item)", code_writer);
                    });
                    code_writer.add_line("}".to_string());
                    code_writer.add_line("vec".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForArray {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Tuple(ref item_types) => {
                code_writer.add_line(format!("{}if let yass::Value::Array(ref items) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line(format!("if items.len() != {} {{", item_types.len()));
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line("return Err(yass_schema_error::Error::InvalidNumberOfTupleElements {".to_string());
                        code_writer.with_indent(|code_writer| {
                            code_writer.add_line("num_elements: items.len(),".to_string());
                            code_writer.add_line(format!("num_expected: {},", item_types.len()));
                            code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                        });
                        code_writer.add_line("});".to_string());
                    });
                    code_writer.add_line("}".to_string());
                    code_writer.add_empty_line();
                    
                    let mut ret_str = "(".to_string();
                    for (i, item_type) in item_types.iter().enumerate() {
                        let prefix = format!("let item_{} = ", i);
                        let item_value = format!("(*items[{}])", i);
                        self.gen_from_yass_value(item_type, prefix.as_str(), ";", item_value.as_str(), code_writer);
                        
                        if i != 0 {
                            ret_str.push_str(", ");
                        }
                        use std::fmt::Write;
                        write!(ret_str, "item_{}", i).unwrap();
                    }
                    ret_str.push(')');
                    code_writer.add_line(ret_str);
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForTuple {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Dictionary(ref item_type) => {
                code_writer.add_line(format!("{}if let yass::Value::Struct(ref items) = {} {{", prefix, value));
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("let mut vec = Vec::with_capacity(items.len());".to_string());
                    code_writer.add_line("for item in items {".to_string());
                    code_writer.with_indent(|code_writer| {
                        self.gen_from_yass_value(item_type, "vec.push((item.key.clone(), ", "));", "(*item.value)", code_writer);
                    });
                    code_writer.add_line("}".to_string());
                    code_writer.add_line("vec".to_string());
                });
                code_writer.add_line("} else {".to_string());
                code_writer.with_indent(|code_writer| {
                    code_writer.add_line("return Err(yass_schema_error::Error::InvalidValueTypeForDictionary {".to_string());
                    code_writer.with_indent(|code_writer| {
                        code_writer.add_line(format!("value_pos: pos_map.get_value_pos(&{}),", value));
                    });
                    code_writer.add_line("});".to_string());
                });
                code_writer.add_line(format!("}}{}", suffix));
            }
            schema::Type::Defined(ref type_name) => {
                let code_name = self.type_def_name_map.get(type_name).unwrap();
                code_writer.add_line(format!("{}{}::from_yass_value(&{}, pos_map)?{}", prefix, code_name, value, suffix));
            }
            schema::Type::Boxed(ref item_type) => {
                let prefix = format!("{}Box::new(", prefix);
                let suffix = format!("){}", suffix);
                self.gen_from_yass_value(item_type, prefix.as_str(), suffix.as_str(), value, code_writer);
            }
        }
    }
}

#[inline]
pub fn gen_code_for_schema(schema: &schema::Schema, output: &mut String) {
    CodeGen::gen_code_for_schema(schema, output);
}

#[inline]
pub fn gen_code_for_schema_as_string(schema: &schema::Schema) -> String {
    let mut output = String::new();
    gen_code_for_schema(schema, &mut output);
    output
}

// CodeWriter
struct CodeWriter {
    lines: Vec<(u32, String)>,
    indent: u32,
}

impl CodeWriter {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent: 0,
        }
    }
    
    #[inline]
    fn inc_indent(&mut self, n: u32) {
        self.indent += n;
    }
    
    #[inline]
    fn dec_indent(&mut self, n: u32) {
        self.indent -= n;
    }
    
    #[inline]
    fn with_indent<F>(&mut self, f: F)
        where F: FnOnce(&mut Self)
    {
        self.inc_indent(1);
        f(self);
        self.dec_indent(1);
    }
    
    #[inline]
    fn add_line(&mut self, line: String) {
        self.lines.push((self.indent, line));
    }
    
    #[inline]
    fn add_empty_line(&mut self) {
        self.add_line(String::new());
    }
    
    fn write_to(&self, output: &mut String) {
        for &(indent, ref line) in self.lines.iter() {
            for _ in 0 .. indent {
                output.push_str("    ");
            }
            output.push_str(line.as_str());
            output.push('\n');
        }
    }
}
