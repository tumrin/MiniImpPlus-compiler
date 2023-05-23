# Parser implementation

The parser handles each element in the MiniImp language as a token, takes into account both the previous and upcoming token and evaluates the current token. The current token is then translated into a valid expression in the "destination" language, for example Javascript. The token, which is evaluated by the function ```handle_token()```, is identified as one of the MiniImpPlus enum's types and converted to the language we're compiling to according to that language's definition of the token. These can be found in each the respective files of each language, under the languages folder. The ready, compiled programs are written into files named "output" with their respective file types and contain the freshly-baked source code ready to be run.

The parser traverses through the token in an index-by-index way, i.e. in order. Each of the token is printed to the terminal, so the user can basically see the destination language being compiled.

## Trying out the compilation
If you want to see how the MiniImp code is compiled to either of the exercise languages, it can be done by running the command ```cargo run -- javascript``` or ```cargo run -- rust``` in the terminal. This will produce the "output" files.