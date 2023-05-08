use std::borrow::Cow;

use antlr_rust::{
    common_token_stream::CommonTokenStream, int_stream::IntStream, token::GenericToken,
    InputStream, Parser,
};
use mini_imp::miniimpparser::MiniImpParser;

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
        handle_token(previous_token.clone(), current.clone(), stream.la(2));
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
    next: isize,
) {
    match current.token_type {
        1 => {
            print!("true");
        }
        2 => {
            print!("false");
        }
        3 => {
            print!("!");
        }
        4 => {
            if next == 23 {
                // Skip if next would be variable
            } else if previous.map_or(false, |prev| prev.token_type == 3) {
                // Only use one equal sign for negation
                print!("=");
            } else {
                print!("==");
            }
        }
        5 => {
            print!("+");
        }
        6 => {
            print!("-");
        }
        7 => {
            print!("*");
        }
        8 => {
            print!("/");
        }
        9 => {
            print!("(");
        }
        10 => {
            print!(")");
        }
        11 => {
            print!("if");
        }
        12 => {
            print!("then");
        }
        13 => {
            print!("else");
        }
        14 => {
            print!("while ");
        }
        15 => {
            // Skip set
        }
        16 => {
            print!(" = ");
        }
        17 => {
            print!(";\n");
        }
        18 => {
            print!("println!");
        }
        19 => {
            print!("let mut ");
        }
        20 => {
            print!(" {{\n");
        }
        21 => {
            print!("}}\n");
        }
        22 => {
            print!("fn main()");
        }
        23 => previous.clone().map_or_else(
            || print!("{}", current.text),
            |prev| match prev.token_type {
                18 => {
                    print!("(\"{{{}}}\")", current.text);
                }
                4 => {
                    print!("({} == ", current.text)
                }
                22 => (),
                23 => print!("{})", current.text),
                _ => print!("{}", current.text),
            },
        ),
        24 => {
            print!("{}", current.text);
        }
        25 => {
            print!("WS");
        }
        _ => {
            //print!("unknown");
        }
    }
}

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
