use crate::Rule;

struct Program {
    nodes: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int,
    String, // this one doesn't actually work yet...
}

impl Type {
    fn from_str(s: &str) -> Self {
        match s {
            "Int" => Type::Int,
            "String" => Type::String,
            _ => panic!("Unknown type: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    name: String,
    args: Vec<(String, Type)>,
    return_type: Option<Type>,
    body: Vec<ASTNode>,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        args: Vec<(String, Type)>,
        body: Vec<ASTNode>,
        return_type: Option<Type>,
    ) -> Self {
        FunctionDefinition {
            name,
            args,
            body,
            return_type,
        }
    }

    pub fn check_compatibility(&self, other: &FunctionCall) -> bool {
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

    pub fn get_return_type(&self) -> Option<Type> {
        self.return_type
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<(String, Type)> {
        &self.args
    }

    pub fn get_body(&self) -> &Vec<ASTNode> {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
    args: Vec<ASTNode>,
}

impl FunctionCall {
    pub fn new(name: String, args: Vec<ASTNode>) -> Self {
        FunctionCall { name, args }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<ASTNode> {
        &self.args
    }
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    FunctionDefinition(FunctionDefinition),
    Return(Box<ASTNode>),
    Expression(Expression),
    IfStatement(IfStatement),
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    condition: Box<Expression>,
    body: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Dyadic(Dyadic),
    Value(Value),
    ParenExpression(Box<Expression>),
    Identifier(String),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct Dyadic {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

impl Dyadic {
    pub fn get_left(&self) -> &Expression {
        &self.left
    }

    pub fn get_right(&self) -> &Expression {
        &self.right
    }

    pub fn get_operator(&self) -> &Operator {
        &self.operator
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    String(String),
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
            let mut return_type = None;
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
                            let type_enum = Type::from_str(type_str);
                            args.push((name, type_enum));
                        }
                    }
                    Rule::return_type => {
                        let inner = inner_pair.into_inner();
                        for inner_pair in inner {
                            match inner_pair.as_rule() {
                                Rule::value_type => {
                                    return_type = Some(Type::from_str(inner_pair.as_str()))
                                }
                                _ => {}
                            };
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
                                println!("Error parsing AST for function body: {:?}", inner_clone);
                            }
                        }
                    }
                    _ => (),
                }
            }

            Some(ASTNode::FunctionDefinition(FunctionDefinition::new(
                name,
                args,
                body,
                return_type,
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
            Some(ASTNode::Expression(Expression::FunctionCall(
                FunctionCall { name, args },
            )))
        }
        Rule::return_statement => {
            let inner_pair = pair.into_inner().next()?;
            Some(ASTNode::Return(Box::new(build_ast(inner_pair)?)))
        }
        Rule::expression => {
            let inner_pair = pair.into_inner().next()?;
            let inner_ast = build_ast(inner_pair);
            inner_ast
        }
        Rule::primary_expression => {
            let inner_pair = pair.into_inner().next()?;
            let inner_ast = build_ast(inner_pair);
            inner_ast
        }
        Rule::dyadic => {
            let mut inner_pairs = pair.into_inner();

            let left_ast =
                build_ast(inner_pairs.next().expect("failed to get left dyadic")).unwrap();
            let left: Expression = match left_ast {
                ASTNode::Expression(p) => p,
                _ => panic!("Expected expression"),
            };

            let op_str = inner_pairs
                .next()
                .expect("failed to get operator of dyadic")
                .as_str();

            let operator = match op_str {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                "==" => Operator::Equal,
                "!=" => Operator::NotEqual,
                "<" => Operator::LessThan,
                "<=" => Operator::LessThanOrEqual,
                ">" => Operator::GreaterThan,
                ">=" => Operator::GreaterThanOrEqual,
                _ => panic!("Unknown operator {:?}", op_str),
            };

            let right_ast =
                build_ast(inner_pairs.next().expect("failed to get right of dyadic")).unwrap();
            let right: Expression = match right_ast {
                ASTNode::Expression(p) => p,
                _ => panic!("Expected expression got {:?}", right_ast),
            };

            Some(ASTNode::Expression(Expression::Dyadic(Dyadic {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })))
        }
        Rule::primary_identifier => {
            let name = pair.as_str().to_string();
            Some(ASTNode::Expression(Expression::Identifier(name)))
        }
        Rule::value => {
            let inner_pair = pair.into_inner().next().unwrap();
            let value = match inner_pair.as_rule() {
                Rule::int => Value::Int(inner_pair.as_str().parse().unwrap()),
                Rule::string => Value::String(inner_pair.as_str().to_string()),
                _ => panic!("Unknown value type {:?}", inner_pair.as_rule()),
            };
            Some(ASTNode::Expression(Expression::Value(value)))
        }
        Rule::if_statement => {
            let mut inner_pairs = pair.into_inner();
            let condition = build_ast(inner_pairs.next().unwrap()).unwrap();
            let mut body = vec![];
            while let Some(inner_pair) = inner_pairs.next() {
                let inner_clone = inner_pair.clone(); // clone for debug TODO: remove
                let ast = build_ast(inner_pair);
                if let Some(ast) = ast {
                    body.push(ast);
                } else {
                    println!("Error parsing AST for if statement: {:?}", inner_clone);
                }
            }
            Some(ASTNode::IfStatement(IfStatement {
                condition: Box::new(match condition {
                    ASTNode::Expression(p) => p,
                    _ => panic!("Expected expression"),
                }),
                body,
            }))
        }
        Rule::EOI => None,
        _ => {
            println!("Unknown rule: {:?}", pair.as_rule());
            None
        }
    }
}
