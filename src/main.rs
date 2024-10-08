#[macro_use]
extern crate pest_derive;

use pest::Parser;
mod ast;
use ast::*;
mod spice_translator;
mod sub_circuits;
#[cfg(test)]
mod tests;
mod translator;

#[derive(Parser)]
#[grammar = "ACL.pest"] // Specifies the grammar file
struct HLHDLParser;

const FILE: &str = "test_scripts/adder_with_assignment.acl";

fn main() {
    let unparsed_file = std::fs::read_to_string(FILE).expect("cannot read file");

    let parse_result = HLHDLParser::parse(Rule::program, &unparsed_file);

    match parse_result {
        Ok(parsed) => {
            let mut all_nodes = vec![];
            for pair in parsed {
                let ast = build_ast(pair.clone());
                if let Some(ast) = ast {
                    all_nodes.push(ast);
                }
                // none is returned for like EOI
            }
            let node = ASTNode::Program(all_nodes);
            println!("{:#?}", node);

            let mut translator = translator::Translator::new();
            let circuit = translator.translate_ast(node);
            println!("{:#?}", circuit);
        }
        Err(e) => println!("Parsing error: {}", e),
    }
}
