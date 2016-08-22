//! `Ports` are the IO channels for a `Node`
//!
//! They can be used as a `Source` and `Destination` for certain `Instruction`s.

/// A `Port` can be read from and write to
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Port {
    /// The input port
    pub input: Vec<i32>,
    /// the output port
    pub output: Vec<i32>,
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
    pub fn read(&self) -> Option<(Port,i32)> {
        if self.input.len() > 0 {
            let mut result_input = vec![];
            for index in 1..self.input.len() {
                result_input.push(self.input[index]);
            }
            Some((Port::with(result_input, vec![]), self.input[0]))
        } else {
            None
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
            Some((next_port, value)) => {
                assert_eq!(0, value);
                assert_eq!(Port::with(vec![1,2], vec![]), next_port);
            },
            None => panic!(),

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
