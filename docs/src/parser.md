# Parser implementation

Parser uses the visitor pattern to traverese each node and produce code in the destination
language matching that node. We select the language by using command line argument parsed
by [Clap](https://crates.io/crates/clap) crate. The argument is mapped to languages enum listing all available languages.
The file type is also selected with this enum.

The parsing is done by [antlr\_rust](https://crates.io/crates/antlr-rust) crate. Every language supported implements ***ParseTreeVisitorCompat*** trait and
the antlr generated ***MiniImpVisitorCompat*** trait which allows running code when certain type of node is visited.
The traits are implemented for a struct which has a single string field. This string is the return value of the
visitor which contains translated source code of that language.

The end result of the parsing is written to a file titled **output.\<filetype\>**.
