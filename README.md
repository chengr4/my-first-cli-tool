# my-first-cli-tool

- Learn from https://github.com/rust-cli/book

## Have Learned

- `BufReader` is a better choise than `read_to_string`
- Instead of the `panic!`, we can also easily write `return`

```rust
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
```

- `unwrap()` is a shortcut for the match with `panic!`
- `Box<dyn std::error::Error>`: It’s a Box that can contain any type that implements the standard `Error` trait. All errors can be put into this box

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `?` expands to code that converts error types and returns error
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}

```

- `println!` tells the system to flush to the terminal every time (from buffer to terminal)
- By convention, cargo will look for integration tests in the tests/ directory.

## References

- [Crate clap](https://docs.rs/clap/latest/clap/)
