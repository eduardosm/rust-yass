// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate yass_parser;
extern crate yass_serializer;

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [compact|spaced] [input-file]", args[0].to_string_lossy());
        std::process::exit(1);
    }
    
    let serialize_style = match &*args[1].to_string_lossy() {
        "compact" => yass_serializer::SerializeStyle::Compact,
        "spaced" => yass_serializer::SerializeStyle::Spaced {
            line_break_type: yass_serializer::LineBreakType::Lf,
            indent_type: yass_serializer::IndentType::Space,
            indent_length: 2,
        },
        _ => {
            eprintln!("Unknown style {:?}.", args[1]);
            std::process::exit(1);
        }
    };
    
    let parser_limits = yass_parser::ParserLimits::unlimited();
    let parser_output = yass_parser::parse_file(parser_limits, &args[2]).unwrap().1;
    let reserialized = serialize_style.serialize_as_string(&parser_output);
    
    println!("{}", reserialized);
}
