//! turns functions into circuits

use std::{collections::HashMap, hash::Hash};

use crate::{ASTNode, FunctionDefinition};

pub struct Circuit {
    parts: Vec<Box<dyn Part>>,
    connections: Vec<(usize, usize)>,
    inputs: Vec<(usize, usize)>,  // (part_index, input_index)
    outputs: Vec<(usize, usize)>, // (part_index, output_index)
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            parts: vec![],
            connections: vec![],
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn add_part(&mut self, part: Box<dyn Part>) {
        self.parts.push(part);
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.connections.push((from, to));
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

struct Translator {
    scope_defs: Vec<ScopeInfo>,
}

impl Translator {
    /// we copy the last scope whenever we enter a new scope
    fn enter_scope(&mut self) {
        self.scope_defs
            .push(self.scope_defs.last().unwrap().clone());
    }

    /// we drop the last scope whenever we exit a scope
    fn exit_scope(&mut self) {
        self.scope_defs.pop();
    }

    fn translate_function_def(&mut self, node: FunctionDefinition) -> Circuit {
        let mut circuit = Circuit::new();

        // translate the body of the function
        for sub_node in node.get_body() {
            match sub_node {
                ASTNode::Return(_) => {
                    // get the circuit for the expression
                    let internal_circuit = self.translate_ast(sub_node.clone());

                    let index = circuit.parts.len();
                    // add the internal circuit to the main circuit
                    circuit.add_part(Box::new(internal_circuit));

                    // return statement means this is the output of the circuit
                    // connect the output of the internal circuit to the output of the main circuit
                    circuit.connect(index, circuit.outputs[0].0);
                }
                _ => {
                    let sub_circuit = self.translate_ast(sub_node.clone());
                    circuit.add_part(Box::new(sub_circuit));
                }
            }
        }

        circuit
    }

    fn translate_if_statement(&self, node: ASTNode) -> Circuit {
        let mut circuit = Circuit::new();

        // translate the body of the function

        circuit
    }

    fn translate_ast(&mut self, node: ASTNode) -> Circuit {
        match node {
            ASTNode::Program(nodes) => {
                let mut circuit = Circuit::new();
                for node in nodes {
                    let sub_circuit = self.translate_ast(node);
                    for part in sub_circuit.parts {
                        circuit.add_part(part);
                    }
                }
                circuit
            }
            ASTNode::FunctionDefinition(func_def) => {
                self.enter_scope();
                let node = self.translate_function_def(func_def);
                self.exit_scope();
                node
            }
            ASTNode::IfStatement(_) => {
                self.enter_scope();
                let node = self.translate_if_statement(node);
                self.exit_scope();
                node
            }
            ASTNode::Return(_) => {
                panic!("return statement not handled by function definition")
            }
            _ => unimplemented!(),
        }
    }
}
