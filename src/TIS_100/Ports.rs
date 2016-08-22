//! `Ports` are the IO channels for a `Node`
//!
//! They can be used as a `Source` and `Destination` for certain `Instruction`s.

/// A `Port` can be read from and write to
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Port {
    input: Vec<i32>,
    output: Vec<i32>,
}

/// The result of reading from a `Port`
#[derive(Debug)]
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

    /// Create a port with prescribed input and output
    pub fn with(input: Vec<i32>, output: Vec<i32>) -> Port {
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

    /// Write to this `Port`. Will always succeed and return the Port as it is
    /// changed by the write
    pub fn write(&self, value: i32) -> Port {
        let mut result_input = vec![];
        for index in 0..self.input.len() {
            result_input.push(self.input[index]);
        }
        let mut result_output = vec![];
        for index in 0..self.output.len() {
            result_output.push(self.output[index]);
        }
        result_output.push(value);
        Port::with(result_input, result_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_a_port_should_work_correctly() {
        let port: Port = Port::new(vec![0, 1, 2]);

        match port.read() {
            PortReadResult::Success(next_port, value) => {
                assert_eq!(0, value);
                assert_eq!(Port::with(vec![1,2], vec![]), next_port);
            },
            PortReadResult::Failure => panic!(),

        }
    }

    #[test]
    fn write_to_a_port_should_work_correctly() {
        let port: Port = Port::with(vec![1, 2], vec![3]);

        let next_port = port.write(4);

        assert_eq!(Port::with(vec![1, 2], vec![3, 4]), next_port);
    }

    #[test]
    fn ports_should_be_able_to_be_cloned() {
        let port: Port = Port::with(vec![1, 2], vec![3, 4]);

        let clone: Port = port.clone();

        assert_eq!(clone, port);
    }
}
