# Javascript

To compile the output.mjs file, the translator has to be run with a javascript flag.

```bash
cargo run -- javascript
```

The Javascript program can be run from the command line using [NodeJS](https://nodejs.org/en).
Please make sure to use node major version 18, as the compiled
program may not work on
older versions as it requires support for top level async.

Running the program can be done by using the command

```bash
node output.mjs
```

Due to the eventloop of node, the readline was hard to use with synchronously and had to be implemented with async functions.
