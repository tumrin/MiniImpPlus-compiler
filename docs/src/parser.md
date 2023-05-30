# Parser implementation

Parser uses the visitor pattern to traverese each node and produce code in the destination
language matching that node. We select the language by using command line argument
parsed by [Clap](https://crates.io/crates/clap) crate.
The argument is mapped to languages enum listing all available languages.
The file type is also selected with this enum.

The parsing is done by [antlr\_rust](https://crates.io/crates/antlr-rust) crate.
Every language supported implements ***ParseTreeVisitorCompat*** trait and
the antlr generated ***MiniImpVisitorCompat*** trait which allows running
code when certain type of node is visited. We only need to change the nodes that are different in target language.
Other nodes are left with default implementation which is just calling visit children and returning the output of that
Antlr itself does not support generating
Rust code so we used a [fork](https://github.com/rrevenantt/antlr4/tree/rust-target) with Rust support.

The traits are implemented for a struct which has a single string field.
This string is the return value of the
visitor which contains translated source code of that language.

The end result of the parsing is written to a file titled **output.\<filetype\>**.

## Issues

The parser gives some debug warnings, but they don't affect the output source code and the programs work fine.
