#[macro_use]
extern crate pest_derive;

use pest::Parser;
mod ast;
use ast::*;
mod sub_circuits;
mod translator;

#[derive(Parser)]
#[grammar = "ACL.pest"] // Specifies the grammar file
struct HLHDLParser;

const FILE: &str = "test_scripts/basic_adder_example.acl";

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

#[cfg(test)]
mod tests {
    use super::*;

    // test the test scripts compile
    #[test]
    fn test_all_scripts() {
        // find all the files in the test_scripts directory
        let paths = std::fs::read_dir("test_scripts").unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            println!("Testing file: {}", path);
            let unparsed_file = std::fs::read_to_string(path).expect("cannot read file");

            let parse_result = HLHDLParser::parse(super::Rule::program, &unparsed_file);

            match parse_result {
                Ok(parsed) => {
                    let mut all_nodes = vec![];
                    for pair in parsed {
                        let ast = super::build_ast(pair.clone());
                        if let Some(ast) = ast {
                            all_nodes.push(ast);
                        }
                        // none is returned for like EOI
                    }
                    let node = super::ASTNode::Program(all_nodes);

                    let mut translator = super::translator::Translator::new();
                    let circuit = translator.translate_ast(node);
                    println!("{:#?}", circuit);
                }
                Err(e) => println!("Parsing error: {}", e),
            }
        }
    }
}
