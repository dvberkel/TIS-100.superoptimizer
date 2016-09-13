```rust
match index.rem(19) {
  1 => Instruction::SWP,
  2 => Instruction::SAV,
  3 => Instruction::ADD(Source::Port),
  4 => Instruction::ADD(Source::Register(Register::ACC)),
  5 => Instruction::ADD(Source::Literal(1)),
  6 => Instruction::SUB(Source::Port),
  // Some instructions left out
  16 => Instruction::MOV(Source::Literal(0), Destination::Register(Register::ACC)),
  17 => Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)),
  18 => Instruction::MOV(Source::Literal(-1), Destination::Register(Register::ACC)),
  _ => Instruction::NOP,
}
```
