# Rust

After running the translator with the rust flag.

```bash
cargo run -- --rust
```

an output.rs file is created. This file can then be compliled by invoking
the rustc directly.

```bash
rustc output.rs
```

This results in a binary file that can be run as any other program.

```bash
./output
```

## Implementation details

As Rust requires variables to explicitly state that they are mutable with
"mut" keyword, all variables are assumed to be mutable when translating from
MiniImpPlus to allow set statement to change variable value.
