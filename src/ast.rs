use crate::Rule;

struct Program {
    nodes: Vec<ASTNode>,
}

#[derive(Debug)]
enum Type {
    Int,
    String,
}

#[derive(Debug)]
struct FunctionDefinition {
    name: String,
    args: Vec<(String, Type)>,
    body: Vec<ASTNode>,
}

impl FunctionDefinition {
    fn new(name: String, args: Vec<(String, Type)>, body: Vec<ASTNode>) -> Self {
        FunctionDefinition { name, args, body }
    }

    fn check_compatibility(&self, other: &FunctionCall) -> bool {
        if self.name != other.name {
            return false;
        }

        // quick check for number of arguments
        if self.args.len() != other.args.len() {
            return false;
        }

        // check for argument types
        unimplemented!("Check for argument types in function calls");

        //true
    }
}

#[derive(Debug)]
struct FunctionCall {
    name: String,
    args: Vec<ASTNode>,
}

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    FunctionDefinition(FunctionDefinition),
    FunctionCall(FunctionCall),
    Return(Box<ASTNode>),
}

pub fn build_ast(pair: pest::iterators::Pair<Rule>) -> Option<ASTNode> {
    match pair.as_rule() {
        Rule::program => {
            let mut nodes = vec![];
            for inner_pair in pair.into_inner() {
                nodes.push(build_ast(inner_pair)?);
            }
            Some(ASTNode::Program(nodes))
        }
        Rule::function_def => {
            let mut inner_pairs = pair.into_inner();
            let name = inner_pairs.next().unwrap().as_str().to_string();
            let mut args = vec![];
            let mut body = vec![];
            // keep consuming pairs until we reach the function body
            while let Some(inner_pair) = inner_pairs.next() {
                match inner_pair.as_rule() {
                    Rule::params => {
                        let inner_pairs = inner_pair.into_inner();
                        for inner_pair in inner_pairs {
                            let param;
                            match inner_pair.as_rule() {
                                Rule::param => param = inner_pair,
                                _ => continue,
                            }
                            let mut inner_pairs = param.into_inner();
                            let name = inner_pairs.next().unwrap().as_str().to_string();
                            let type_str = inner_pairs.next().unwrap().as_str();
                            let type_enum = match type_str {
                                "Int" => Type::Int,
                                "String" => Type::String,
                                _ => {
                                    panic!("Unknown type: {}", type_str);
                                }
                            };
                            args.push((name, type_enum));
                        }
                    }
                    Rule::function_body => {
                        let mut inner_pairs = inner_pair.into_inner();
                        // all inner statements
                        while let Some(inner_pair) = inner_pairs.next() {
                            let inner_clone = inner_pair.clone(); // clone for debug TODO: remove
                            let ast = build_ast(inner_pair);
                            if let Some(ast) = ast {
                                body.push(ast);
                            } else {
                                println!("Error parsing AST: {:?}", inner_clone);
                            }
                        }
                    }
                    _ => (),
                }
            }

            println!("name: {:?}", name);
            println!("args: {:?}", args);

            let body = vec![];
            Some(ASTNode::FunctionDefinition(FunctionDefinition::new(
                name, args, body,
            )))
        }
        Rule::function_call => {
            let mut inner_pairs = pair.into_inner();
            let name = inner_pairs.next().unwrap().as_str().to_string();
            let args = inner_pairs
                .next()
                .unwrap()
                .into_inner()
                .map(|inner_pair| build_ast(inner_pair))
                .collect::<Option<Vec<ASTNode>>>()?;
            Some(ASTNode::FunctionCall(FunctionCall { name, args }))
        }
        Rule::return_statement => {
            let inner_pair = pair.into_inner().next()?;
            Some(ASTNode::Return(Box::new(build_ast(inner_pair)?)))
        }
        _ => None,
    }
}
