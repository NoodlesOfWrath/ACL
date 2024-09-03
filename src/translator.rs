//! turns functions into circuits

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

struct Translator {}

impl Translator {
    fn translate_function_def(&self, node: FunctionDefinition) -> Circuit {
        unimplemented!()
    }

    fn translate_ast(&self, node: ASTNode) -> Circuit {
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
            ASTNode::FunctionDefinition(func_def) => self.translate_function_def(func_def),

            _ => unimplemented!(),
        }
    }
}
