```rust
loop {
    match node.fetch_instruction() {
        Some(instruction) => {
            match node.execute(instruction) {
                Some(next_node) => node = next_node,
                None => return Err(ErrorStatus::Deadlock(node)),
            }
        }
        None => {
            if node.up.available() {
                node = node.set_pc(0);
            } else {
                break;
            }
        }
    }
}
Ok(node)
```
 
