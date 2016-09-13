```rust
pub struct Program(pub Vec<Instruction>);

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Instruction {
    /// Does nothing
    NOP,
    /// Moves value from `Source` to `Destination`
    MOV(Source, Destination),
    /// Swaps the value of the accumulator (acc) and the backup (bac) register
    SWP,
    /// Saves the value of the accumulator (acc) to the backup register
    SAV,
    /// Add value from `Source` to accumulator (acc), storing result in acc
    ADD(Source),
    /// Subtracts value from `Source` from accumulator (acc), storing result in acc
    SUB(Source),
}
```
