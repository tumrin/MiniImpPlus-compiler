# Setup

This program is written in rust and therefore Rust compiler is needed to compile
and run it. Rust can be installed from [The Rust website](https://www.rust-lang.org/tools/install).

## Compiling

```bash
# In the project root
cargo build # build in debug mode
```

## Using the translator

You can see the help menu with:

```bash
cargo run -- --help
```

If you want to see how the MiniImp code is compiled to either of
the exercise languages, it can be done by running the command.

```bash
cargo run -- javascript
# or
cargo run -- rust
```
