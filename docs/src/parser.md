# Parser implementation

The parser handles each element in the MiniImp language as a token, takes into account both the previous and upcoming token and evaluates the current token. The current token is then translated into a valid expression 
in the "destination" language, for example Javascript. The token, which is evaluated by the function ```handle_token()```, is identified as one of the enum's types that the MiniImpPlus tokens have been mapped to
and converted to the language we're compiling to according to that language's definition of the token. These can be found in each the respective files of each language, under the languages folder. The ready, compiled programs are written into files named "output" with 
their respective file types and contain the freshly-baked source code ready to be run.

The translations function is part of a translateMiniImp trait which must be implemeted for all languages that the translator supports. 

The parser traverses through the token in an index-by-index way, i.e. in order. Each of the tokens is printed to the terminal, so the user can basically see the destination language being compiled.

# Why this instead of visitor or listener?
Visitor and listener were both considered but there were issues getting them working and the partly working implementation was much more verbose and contained more boilerplate code compared to current implementation.
These patterns were also poorly documented in the antlr_rust crate although that is on us for choosing to use Rust in this exercise as the antlr crate is currently in beta phase.
