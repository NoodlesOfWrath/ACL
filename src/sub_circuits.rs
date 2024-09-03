use crate::translator::Part;
// ! these are placeholders for now, they should be circuits of transistors or something

struct Multiplier {}

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

struct Adder {}

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
