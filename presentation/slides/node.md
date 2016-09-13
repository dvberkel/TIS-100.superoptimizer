```rust
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Node {
    /// The accumulator for the basic execution node.
    pub acc: i32,
    /// The up `Port` used for reading
    pub up: Port,
    /// The down `Port` used for writing
    pub down: Port,
    bac: i32,
    pc: usize,
    program: Program,
}
```
