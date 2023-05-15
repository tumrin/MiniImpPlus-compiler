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
    let test = "program CALCULATOR\n
    begin\n
    var PI = 3;\n
    var ANSWER = PI;\n
    var X = 9;\n
    \n
    while not is X PI begin\n
    set ANSWER = ANSWER + X;\n
    set X = X - 1;\n
    end.\n
    \n
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
// 'true'=1
// 'false'=2
// 'not'=3
// 'is'=4
// 'or'=5
// 'and'=6
// '+'=7
// '-'=8
// '*'=9
// '/'=10
// '('=11
// ')'=12
// 'if'=13
// 'then'=14
// 'else'=15
// 'while'=16
// 'set'=17
// '='=18
// ';'=19
// 'write'=20
// 'read'=21
// 'var'=22
// 'asNumber'=23
// 'asString'=24
// 'begin'=25
// 'end.'=26
// 'program'=27

// Identifier=23
// Number=24
// WS=25
// 'true'=1
// 'false'=2
// 'not'=3
// 'is'=4
// '+'=5
// '-'=6
// '*'=7
// '/'=8
// '('=9
// ')'=10
// 'if'=11
// 'then'=12
// 'else'=13
// 'while'=14
// 'set'=15
// '='=16
// ';'=17
// 'write'=18
// 'var'=19
// 'begin'=20
// 'end.'=21
// 'program'=22
