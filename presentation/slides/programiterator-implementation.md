```rust
impl Iterator for ProgramIterator {
    type Item = Program;

    fn next(&mut self) -> Option<Program> {
        let instructions: Vec<Instruction> =
            digits_of(self.current, 19)
            .iter()
            .map(|digit| instruction_map(*digit))
            .collect();
        self.current += 1;
        Some(Program(instructions))
    }
}
```
