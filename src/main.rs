#[macro_use]
extern crate pest_derive;

use pest::{pratt_parser::PrattParser, Parser};
mod ast;
use ast::*;

#[derive(Parser)]
#[grammar = "HLHDL.pest"] // Specifies the grammar file
struct HLHDLParser;

fn main() {
    let unparsed_file = std::fs::read_to_string("example.hlhdl").expect("cannot read file");

    let parse_result = HLHDLParser::parse(Rule::program, &unparsed_file);

    match parse_result {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(e) => println!("Parsing error: {}", e),
    }
}
