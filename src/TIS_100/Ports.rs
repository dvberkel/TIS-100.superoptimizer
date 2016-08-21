//! `Ports` are the IO channels for a `Node`
//!
//! They can be used as a `Source` and `Destination` for certain `Instruction`s.

/// A `Port` can be read from and write to
pub struct Port {
    input: Vec<i32>,
    output: Vec<i32>,
}

/// The result of reading from a `Port`
pub enum PortReadResult {
    /// Success
    Success(Port, i32),
    /// Failure
    Failure
}

impl Port {
    /// Create a port with a number of readable values
    pub fn new(input: Vec<i32>) -> Port {
        Port { input: input, output: vec![] }
    }

    fn with(input: Vec<i32>, output: Vec<i32>) -> Port {
        Port { input: input, output: output }
    }

    /// Read from this `Port`. Will return a `PortReadResult::Success` when a
    /// value is available, otherwise a `PortReadResult::Failure`
    pub fn read(&self) -> PortReadResult {
        if self.input.len() > 0 {
            let mut result_input = vec![];
            for index in 1..self.input.len() {
                result_input.push(self.input[index]);
            }
            PortReadResult::Success(Port::with(result_input, vec![]), self.input[0])
        } else {
            PortReadResult::Failure
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_a_port_should_work_correctly() {
        let port: Port = Port::new(vec![0, 1, 2]);

        match port.read() {
            PortReadResult::Success(_, value) => assert_eq!(0, value),
            PortReadResult::Failure => panic!(),

        }
    }
}
