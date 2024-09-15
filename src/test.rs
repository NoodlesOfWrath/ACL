use super::*;
use test_each_file::test_each_file;

test_each_file! { in "./test_scripts" => test_script }

// test the test scripts compile
fn test_script(unparsed_file: &str) {
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
        Err(e) => {
            panic!("Parsing error: {}", e);
        }
    }
}
