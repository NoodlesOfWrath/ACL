//! turns functions into circuits

use std::{collections::HashMap, hash::Hash};

use crate::{
    sub_circuits::{Adder, Divider, Multiplier, Subtractor},
    ASTNode, Expression, FunctionDefinition, Operator,
};

pub struct Circuit {
    parts: Vec<Box<dyn Part>>,
    connections: Vec<(usize, usize)>,
    inputs: Vec<(usize, usize)>,  // (part_index, input_index)
    outputs: Vec<(usize, usize)>, // (part_index, output_index)
    // I don't really like the idea of having to keep track of the next input/output index
    // but I can't think of a better way to do it other than summing the input/output sizes of all the parts every single time
    next_input_index: usize,
    next_output_index: usize,
}

struct PartInfo {
    input_offset: usize, // the first input will be at input_offset then the next will be at input_offset + 1 etc
    output_offset: usize, // the same as input_offset but for outputs
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            parts: vec![],
            connections: vec![],
            inputs: vec![],
            outputs: vec![],
            next_input_index: 0,
            next_output_index: 0,
        }
    }

    fn add_part(&mut self, part: Box<dyn Part>) -> PartInfo {
        let part_info = PartInfo {
            input_offset: self.next_input_index,
            output_offset: self.next_output_index,
        };

        // get the size of the inputs and outputs of the part
        let input_size = part.get_input_size();
        let output_size = part.get_output_size();
        // update the next input/output index
        self.next_input_index += input_size;
        self.next_output_index += output_size;

        self.parts.push(part);

        part_info
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.connections.push((from, to));
    }

    // add an input to the circuit
    fn add_input(&mut self) -> usize {
        let input_index = self.next_input_index;
        self.next_input_index += 1;
        input_index
    }

    // add an output to the circuit
    fn add_output(&mut self) -> usize {
        let output_index = self.next_output_index;
        self.next_output_index += 1;
        output_index
    }
}

impl Part for Circuit {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        "Circuit".to_string()
    }

    fn get_input_size(&self) -> usize {
        0
    }

    fn get_output_size(&self) -> usize {
        0
    }
}

pub trait Part {
    fn test(&self, input: Vec<f64>) -> Vec<f64>;
    fn get_name(&self) -> String;
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
}

struct Constant {
    value: f64,
}

impl Part for Constant {
    fn test(&self, _input: Vec<f64>) -> Vec<f64> {
        vec![self.value]
    }

    fn get_name(&self) -> String {
        "Constant".to_string()
    }

    fn get_input_size(&self) -> usize {
        0
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

struct Resistor {
    resistance: f64, // ohms
}

impl Part for Resistor {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        // im pretty sure this is just wrong...
        vec![input[0] / self.resistance]
    }

    fn get_name(&self) -> String {
        "Resistor".to_string()
    }

    fn get_input_size(&self) -> usize {
        1
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Clone)]
struct VariableInfo {
    index: usize,
}

#[derive(Clone)]
struct ScopeInfo {
    variables: HashMap<String, VariableInfo>,
}

pub struct Translator {
    scope_defs: Vec<ScopeInfo>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            scope_defs: vec![ScopeInfo {
                variables: HashMap::new(),
            }],
        }
    }

    /// get the index of a variable in the current scope
    pub fn get_variable_index(&mut self, ident: String) -> usize {
        let scope = self.scope_defs.last().unwrap();
        let var_info = scope.variables.get(&ident).unwrap();
        var_info.index
    }

    /// we copy the last scope whenever we enter a new scope
    fn enter_scope(&mut self) {
        self.scope_defs
            .push(self.scope_defs.last().unwrap().clone());
    }

    /// we drop the last scope whenever we exit a scope
    fn exit_scope(&mut self) {
        self.scope_defs.pop();
    }

    fn translate_function_def(
        &mut self,
        node: FunctionDefinition,
        circuit: &mut Circuit,
    ) -> Option<usize> {
        // add the inputs of the function to the circuit
        for input in node.get_args() {
            let input_index = circuit.add_input();
            let var_info = VariableInfo { index: input_index };
            let name = input.0.clone();
            // the type isn't used for now
            self.scope_defs
                .last_mut()
                .unwrap()
                .variables
                .insert(name, var_info);
        }

        let mut output_index = None;

        // translate the body of the function
        for sub_node in node.get_body() {
            match sub_node {
                ASTNode::Return(_) => {
                    // get the circuit for the expression
                    let internal_output_index =
                        self.translate_ast_internal(sub_node.clone(), circuit);

                    // return statement means this is the output of the circuit
                    // connect the output of the internal circuit to the output of the main circuit
                    let new_output_index = circuit.add_output();
                    circuit.connect(internal_output_index, new_output_index);
                    output_index = Some(new_output_index);
                }
                _ => {
                    let _output_index = self.translate_ast_internal(sub_node.clone(), circuit);
                }
            }
        }

        output_index
    }

    fn translate_if_statement(&self, node: ASTNode, circuit: &mut Circuit) -> usize {
        // translate the body of the function
        unimplemented!()
    }

    /// Outputs the index of the output of the circuit
    pub fn translate_ast_internal(&mut self, node: ASTNode, circuit: &mut Circuit) -> usize {
        match node {
            ASTNode::Program(nodes) => {
                let mut circuit = Circuit::new();
                let mut output_index = None;
                for node in nodes {
                    match node {
                        ASTNode::FunctionDefinition(func_def) => {
                            if func_def.get_name() == "main" {
                                if output_index.is_some() {
                                    panic!("main function already defined");
                                }

                                output_index =
                                    Some(self.translate_function_def(func_def, &mut circuit));
                            } else {
                                self.translate_function_def(func_def, &mut circuit);
                            }
                        }
                        _ => (),
                    }
                }

                if output_index.is_none() {
                    panic!("main function not defined");
                }

                0 // TODO: this is a placeholder, it should output the main output of the circuit
            }
            ASTNode::FunctionDefinition(func_def) => {
                self.enter_scope();
                println!("entering scope");
                let output_index = self.translate_function_def(func_def, circuit);
                println!("exiting scope");
                self.exit_scope();
                output_index.expect("don't yet support functions without return statements")
            }
            ASTNode::IfStatement(_) => {
                self.enter_scope();
                let output_index = self.translate_if_statement(node, circuit);
                self.exit_scope();
                output_index
            }
            ASTNode::Return(inner_expr) => {
                let sub_circuit = self.translate_ast(*inner_expr);
                let info = circuit.add_part(Box::new(sub_circuit));
                info.output_offset
            }
            ASTNode::Expression(expr) => self.translate_expression(expr, circuit),
            _ => panic!("{:?} couldn't be handled", node),
        }
    }

    pub fn translate_ast(&mut self, node: ASTNode) -> Circuit {
        let mut circuit = Circuit::new();
        self.translate_ast_internal(node, &mut circuit);
        circuit
    }

    fn translate_expression(&mut self, expr: Expression, circuit: &mut Circuit) -> usize {
        match expr {
            Expression::Dyadic(dyadic) => {
                let left_node = ASTNode::Expression(dyadic.get_left().clone());
                let right_node = ASTNode::Expression(dyadic.get_right().clone());
                let left_circuit_output_index = self.translate_ast_internal(left_node, circuit);
                let right_circuit_output_index = self.translate_ast_internal(right_node, circuit);

                let operator_circuit = self.get_operator_circuit(dyadic.get_operator());
                let operator_info = circuit.add_part(operator_circuit);

                // connect the inputs of the operator to the outputs of the left and right circuits
                circuit.connect(left_circuit_output_index, operator_info.input_offset);
                circuit.connect(right_circuit_output_index, operator_info.input_offset + 1);

                operator_info.output_offset // assuming the operator has only one output
            }
            Expression::Identifier(ident) => {
                let var_index = self.get_variable_index(ident);
                // output the index of the variable
                var_index
            }
            _ => panic!("{:#?} not yet implemented", expr),
        }
    }

    fn get_operator_circuit(&self, operator: &Operator) -> Box<dyn Part> {
        match operator {
            Operator::Plus => Box::new(Adder {}),
            Operator::Minus => Box::new(Subtractor {}),
            Operator::Multiply => Box::new(Multiplier {}),
            Operator::Divide => Box::new(Divider {}),
            _ => panic!("{:?} not yet implemented", operator),
        }
    }
}
