//! turns functions into circuits

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{
    sub_circuits::{Adder, Divider, Multiplier, Subtractor},
    ASTNode, Expression, FunctionDefinition, IfStatement, Operator,
};

#[derive(Debug, Clone)]
pub struct Circuit {
    parts: Vec<Box<dyn PartInternal>>,
    connections: Vec<(usize, usize)>,
    program_inputs: Vec<usize>,
    program_outputs: Vec<usize>,
    // I don't really like the idea of having to keep track of the next input/output index
    // but I can't think of a better way to do it other than summing the input/output sizes of all the parts every single time
    next_input_index: usize,
    next_output_index: usize,
    name: Option<String>,
}

// an object safe version of Part
trait PartInternal {
    fn test(&self, input: Vec<f64>) -> Vec<f64>;
    fn get_name(&self) -> String;
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
    fn clone_internal(&self) -> Box<dyn PartInternal>;
    fn debug(&self) -> Box<dyn Debug>;
}

impl PartInternal for Box<dyn PartInternal> {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        PartInternal::test(&**self, input)
    }

    fn get_name(&self) -> String {
        PartInternal::get_name(&**self)
    }

    fn get_input_size(&self) -> usize {
        PartInternal::get_input_size(&**self)
    }

    fn get_output_size(&self) -> usize {
        PartInternal::get_output_size(&**self)
    }

    fn clone_internal(&self) -> Box<dyn PartInternal> {
        PartInternal::clone_internal(&**self)
    }

    fn debug(&self) -> Box<dyn Debug> {
        PartInternal::debug(&**self)
    }
}

impl<T> PartInternal for T
where
    T: Part + 'static,
{
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        Part::test(self, input)
    }

    fn get_name(&self) -> String {
        Part::get_name(self)
    }

    fn get_input_size(&self) -> usize {
        Part::get_input_size(self)
    }

    fn get_output_size(&self) -> usize {
        Part::get_output_size(self)
    }

    fn clone_internal(&self) -> Box<dyn PartInternal> {
        let part: Box<dyn PartInternal> = Box::new(self.clone()) as Box<dyn PartInternal>;
        part
    }

    fn debug(&self) -> Box<dyn Debug> {
        Part::as_debug(self)
    }
}

impl Debug for dyn PartInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug().fmt(f)
    }
}

impl Clone for Box<dyn PartInternal> {
    fn clone(&self) -> Self {
        self.clone_internal()
    }
}

impl Part for Circuit {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        // for now just return the input
        input
    }

    fn get_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "Unnamed Circuit".to_string())
    }

    fn get_input_size(&self) -> usize {
        self.next_input_index
    }

    fn get_output_size(&self) -> usize {
        self.next_output_index
    }
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            parts: vec![],
            connections: vec![],
            program_inputs: vec![],
            program_outputs: vec![],
            next_input_index: 0,
            next_output_index: 0,
            name: None,
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn add_part(&mut self, part: impl PartInternal + 'static) -> PartInfo {
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

        self.parts.push(Box::new(part));

        part_info
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.connections.push((from, to));
    }

    // add an input to the circuit
    fn add_program_input(&mut self) -> usize {
        let index = self.next_input_index;
        self.program_inputs.push(index);
        self.next_input_index += 1;
        index
    }

    // add an output to the circuit
    fn add_program_output(&mut self) -> usize {
        let index = self.next_output_index;
        self.program_outputs.push(index);
        self.next_output_index += 1;
        index
    }
}

struct PartInfo {
    input_offset: usize, // the first input will be at input_offset then the next will be at input_offset + 1 etc
    output_offset: usize, // the same as input_offset but for outputs
}

pub trait Part: Debug + Clone
where
    Self: 'static,
{
    fn test(&self, input: Vec<f64>) -> Vec<f64>;
    fn get_name(&self) -> String;
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
    fn as_debug(&self) -> Box<dyn Debug> {
        Box::new(self.clone()) as Box<dyn Debug>
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl ScopeInfo {
    fn new() -> Self {
        ScopeInfo {
            variables: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: String, index: usize) {
        self.variables.insert(name, VariableInfo { index });
    }
}

pub struct Translator {
    scope_defs: Vec<ScopeInfo>,
    function_defs: HashMap<String, Circuit>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            scope_defs: vec![ScopeInfo {
                variables: HashMap::new(),
            }],
            function_defs: HashMap::new(),
        }
    }

    pub fn get_function_circuit(&self, name: String) -> &Circuit {
        self.function_defs
            .get(&name)
            .expect(format!("function {} not defined", name).as_str())
    }

    pub fn add_function_circuit(&mut self, name: String, circuit: Circuit) {
        self.function_defs.insert(name, circuit);
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

    fn get_current_scope(&mut self) -> &mut ScopeInfo {
        // i think expecting here is fine since we should always have a scope if this is being called
        self.scope_defs.last_mut().expect("no scope to get")
    }

    fn make_function_circuit(&mut self, node: FunctionDefinition) -> Circuit {
        let mut circuit = Circuit::new();

        // add the inputs of the function to the circuit
        for input in node.get_args() {
            let input_index = circuit.add_program_input();
            let name = input.0.clone();

            // the type isn't used for now
            self.get_current_scope().add_variable(name, input_index);
        }

        // translate the body of the function
        self.process_function_returns(node.clone(), &mut circuit);

        circuit.set_name(node.get_name().to_string());
        circuit
    }

    fn process_function_returns(&mut self, node: FunctionDefinition, circuit: &mut Circuit) {
        for sub_node in node.get_body() {
            match sub_node {
                ASTNode::Return(_) => {
                    // get the circuit for the expression
                    let internal_output_index =
                        self.translate_ast_internal(sub_node.clone(), circuit);

                    // return statement means this is the output of the circuit
                    // connect the output of the internal circuit to the output of the main circuit
                    let new_output_index = circuit.add_program_output();
                    circuit.connect(internal_output_index, new_output_index);
                }
                _ => {
                    let _output_index = self.translate_ast_internal(sub_node.clone(), circuit);
                }
            }
        }
    }

    fn translate_function_def(
        &mut self,
        node: FunctionDefinition,
        circuit: &mut Circuit,
    ) -> Option<usize> {
        if node.get_name() == "main" {
            // set the circuit to the main circuit
            let main_circuit = self.make_function_circuit(node);
            *circuit = main_circuit;
            Some((circuit as &dyn PartInternal).get_output_size() - 1)
        } else {
            let function_name = node.get_name().to_string();
            let function_circuit = self.make_function_circuit(node);
            println!("name: {:?}", Part::get_name(&function_circuit));
            self.add_function_circuit(function_name, function_circuit);
            None
        }
    }

    // We have to note that if statements are pointless without
    // case 1:
    // - let value = if(smth) {smth} else {smth}
    // which i will ignore for now
    // - a return statement in the if statement
    // which is problematic because return statements are only parsed in the outermost body of a function right now
    // this should form a circuit that looks like this for case 1 (which we aren't supporting for now):
    // condition_circuit   -            - body_circuit -
    //                       \        /                 \
    //                         Gate -                    |
    //                       /        \                  |
    // if statement inputs -            ------------------ rest of the function
    // in case 2 the circuit would look like this
    // condition_circuit   -             ----- body_circuit -----
    //                       \         /                          \
    //                         - Gate -                              - Function Out
    //                       /         \                          /
    // if statement inputs -             - rest of the function -
    fn translate_if_statement(&mut self, node: IfStatement, circuit: &mut Circuit) {
        // two parts: the condition and the body
        let condition_circuit =
            self.translate_ast_internal(ASTNode::Expression(node.get_condition().clone()), circuit);
        let mut body_circuit = Circuit::new();
        for sub_node in node.get_body() {
            let _output_index = self.translate_ast_internal(sub_node.clone(), &mut body_circuit);
            // now we need a function that can look at a section of code and give us a wire that represents the output of that section
            // a "return finder" if you will
        }

        unimplemented!()
    }

    /// Outputs the index of the output of the circuit
    pub fn translate_ast_internal(
        &mut self,
        node: ASTNode,
        circuit: &mut Circuit,
    ) -> Option<usize> {
        match node {
            ASTNode::Program(nodes) => {
                let mut output_index = None;
                for node in nodes {
                    match node {
                        ASTNode::FunctionDefinition(func_def) => {
                            if func_def.get_name() == "main" {
                                if output_index.is_some() {
                                    panic!("main function already defined");
                                }

                                output_index = self.translate_function_def(func_def, circuit);
                            } else {
                                self.translate_function_def(func_def, circuit);
                            }
                        }
                        _ => (),
                    }
                }

                if output_index.is_none() {
                    panic!("main function not defined or doesn't return anything");
                }

                output_index
            }
            ASTNode::FunctionDefinition(func_def) => {
                self.enter_scope();
                let output_index = self.translate_function_def(func_def, circuit);
                self.exit_scope();
                output_index
            }
            ASTNode::IfStatement(statement) => {
                self.enter_scope();
                self.translate_if_statement(statement, circuit);
                self.exit_scope();
                None
            }
            ASTNode::Return(ref inner_expr) => {
                // get the circuit for the expression
                let internal_output_index = self.translate_ast_internal(node.clone(), circuit);

                // return statement means this is the output of the circuit
                // connect the output of the internal circuit to the output of the main circuit
                let new_output_index = circuit.add_program_output();
                circuit.connect(internal_output_index?, new_output_index);
                Some(new_output_index)
            }
            ASTNode::Expression(expr) => Some(self.translate_expression(expr, circuit)),

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
            Expression::FunctionCall(call) => {
                // This will take a lot of thought. Some sort of structure where it can guarentee the function isn't being used twice at the same time
                // And if it is instatiate a new version
                //panic!("Function calls are not yet implemented.")
                let mut arg_indices = vec![];
                for arg in call.get_args() {
                    let arg_index = self.translate_ast_internal(arg.clone(), circuit);
                    arg_indices.push(arg_index);
                }

                // get the function definition
                let function_name = call.get_name();
                let function_circuit = self.get_function_circuit(function_name.to_string());
                let mut function_circuit_clone = function_circuit.clone();
                function_circuit_clone.set_name(function_name.to_string());

                // add the function to the circuit
                let function_info = circuit.add_part(function_circuit.clone());

                // connect the inputs of the function to the outputs of the arguments
                for (i, arg_index) in arg_indices.iter().enumerate() {
                    circuit.connect(*arg_index, function_info.input_offset + i);
                }

                // connect the output of the function to the output of the circuit
                function_info.output_offset
            }
            _ => panic!("{:#?} not yet implemented", expr),
        }
    }

    fn get_operator_circuit(&self, operator: &Operator) -> Box<dyn PartInternal> {
        match operator {
            Operator::Plus => Box::new(Adder {}),
            Operator::Minus => Box::new(Subtractor {}),
            Operator::Multiply => Box::new(Multiplier {}),
            Operator::Divide => Box::new(Divider {}),
            _ => panic!("{:?} not yet implemented", operator),
        }
    }
}
