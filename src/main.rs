use antlr_rust::{
    common_token_stream::CommonTokenStream,
    token::{GenericToken, Token},
    InputStream, Parser,
};
use mini_imp::miniimpparser::MiniImpParser;
use mini_imp_plus::{languages::rust::Rust, MiniImpPlus, TranslateMiniImpPlus};
use std::{borrow::Cow, fs};

mod mini_imp;

fn main() {
    // let test = "program CALCULATOR\n
    // begin\n
    // var PI = 3;\n
    // var ANSWER = PI;\n
    // var X = 9;\n
    // \n
    // while not is X PI begin\n
    // set ANSWER = ANSWER + X;\n
    // set X = X - 1;\n
    // end.\n
    // \n
    // write ANSWER;\n
    // \n
    // end.\n";
    let test = "program CALCULATOR\n
    begin\n
    read ANSWER;\n
    write ANSWER;\n
    \n
    end.\n";

    let stream = InputStream::new(test);
    let lexer = mini_imp::miniimplexer::MiniImpLexer::new(stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = MiniImpParser::new(token_stream);

    let mut previous_token: Option<Box<GenericToken<Cow<str>>>> = None;
    while !parser.matched_eof {
        let current = parser.get_current_token().clone();
        let stream = parser.get_input_stream_mut();

        // if next is not last add 1 to index, yes this is hacky
        let next_index = if stream.la(2) != -1 {
            current.get_token_index() + 1
        } else {
            current.get_token_index()
        };
        let next = stream.get(next_index).clone();

        let token = handle_token(previous_token.clone(), current.clone(), next, &Rust);
        print!("{token}");
        previous_token = Some(current);
        if stream.la(1) != -1 {
            stream.consume();
        } else {
            parser.matched_eof = true;
        }
    }
}

fn handle_token(
    previous: Option<Box<GenericToken<Cow<str>>>>,
    current: Box<GenericToken<Cow<str>>>,
    next: Box<GenericToken<Cow<str>>>,
    language: &impl TranslateMiniImpPlus,
) -> String {
    language.translate(
        MiniImpPlus::from(current),
        previous.map(|token| MiniImpPlus::from(token)),
        MiniImpPlus::from(next),
    )
}
