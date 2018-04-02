// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate yass;
extern crate yass_aux;
extern crate yass_parser;
extern crate yass_schema_error;

mod gen {
    use yass;
    use yass_aux;
    use yass_schema_error;
    
    // See build.rs
    include!(concat!(env!("OUT_DIR"), "/example_1_schema.rs"));
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [input-file]", args[0].to_string_lossy());
        std::process::exit(1);
    }
    
    let parser_limits = yass_parser::ParserLimits::unlimited();
    let (_, parser_output, pos_map) = yass_parser::parse_file(parser_limits, &args[1]).unwrap();
    
    let data = gen::Widget::from_yass_document(&parser_output, &pos_map).unwrap();
    println!("{:#?}", data);
}
