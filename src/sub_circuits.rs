use crate::translator::Part;
// ! these are placeholders for now, they should be circuits of transistors or something

#[derive(Debug, Clone)]
pub struct Multiplier {}

impl Part for Multiplier {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = vec![];
        for i in 0..input.len() {
            output.push(input[i] * input[i + 1]);
        }
        output
    }

    fn get_name(&self) -> String {
        "Multiplier".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Adder {}

impl Part for Adder {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = vec![];
        for i in 0..input.len() {
            output.push(input[i] + input[i + 1]);
        }
        output
    }

    fn get_name(&self) -> String {
        "Adder".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Divider {}

impl Part for Divider {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = vec![];
        for i in 0..input.len() {
            output.push(input[i] / input[i + 1]);
        }
        output
    }

    fn get_name(&self) -> String {
        "Divider".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct Subtractor {}

impl Part for Subtractor {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = vec![];
        for i in 0..input.len() {
            output.push(input[i] - input[i + 1]);
        }
        output
    }

    fn get_name(&self) -> String {
        "Subtractor".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
/// A comparator that outputs 1 if the first input is greater than the second, 0 otherwise
pub struct Comparator {}

impl Part for Comparator {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        vec![]
    }

    fn get_name(&self) -> String {
        "Comparator".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
/// A comparator that outputs 1 if the first input is greater than the second, 0 otherwise
pub struct And {}

impl Part for And {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        vec![]
    }

    fn get_name(&self) -> String {
        "And".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
/// A gate that routes input to output 1 if the control input is 0, and routes the input to output 2 if the control input is 1
/// the first input is the control input
/// the second input is the input to be routed
pub struct IfGate {}

impl Part for IfGate {
    fn test(&self, input: Vec<f64>) -> Vec<f64> {
        if input[0].round() == 0.0 {
            vec![input[1], 0.0]
        } else {
            vec![0.0, input[1]]
        }
    }

    fn get_name(&self) -> String {
        "IfGate".to_string()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    // it only outputs a value to one of the two
    fn get_output_size(&self) -> usize {
        2
    }
}
