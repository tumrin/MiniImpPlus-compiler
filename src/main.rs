use antlr_rust::{
    common_token_stream::CommonTokenStream,
    token::{GenericToken, Token},
    InputStream, Parser,
};
use clap::Parser as ArgParser;
use mini_imp::miniimpparser::MiniImpParser;
use mini_imp_plus::{
    languages::{rust::Rust, Languages},
    MiniImpPlus, TranslateMiniImpPlus,
};
use std::{borrow::Cow, format, fs};

mod mini_imp;

/// Translates form MiniImp to an another language
#[derive(ArgParser, Debug)]
struct Args {
    /// Language to translate to
    #[clap(value_enum)]
    language: Languages,
}

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
    // let test = "program CALCULATOR\n
    // begin\n
    // write \"please insert a number\";
    // read ANSWER;\n
    // write ANSWER;\n
    // \n
    // end.\n";

    let test: &str = "program DEMOAPP \n
    begin \n
        var REPLAY;
        while is REPLAY \"yes\" begin
            var PLAYER_ONE; \n
            var PLAYER_ONE_GESTURE; \n
            var PLAYER_TWO; \n
            var PLAYER_TWO_GESTURE; \n

            write \"Insert name of player 1:\"; \n
            read PLAYER_ONE; \n
            write \"Insert name of player 2:\"; \n
            read PLAYER_TWO; \n

            write \"Starting game with players:\"; \n
            write PLAYER_ONE; \n
            write PLAYER_TWO; \n

            write \"Player 1, choose a gesture (rock/paper/scissors): \";\n
            read PLAYER_ONE_GESTURE; \n

            write \"Player 2, choose a gesture (rock/paper/scissors): \"; \n
            read PLAYER_TWO_GESTURE; \n

            if is PLAYER_TWO_GESTURE PLAYER_ONE_GESTURE then begin write \"Draw\" end.; \n
            
            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write \"Player 2 wins\" end.; \n

            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"paper\" then begin write \"Player 1 wins\" end.; \n

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"paper\" then begin write \"Player 2 wins\" end.; \n

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"rock\" then begin write \"Player 1 wins\" end.; \n

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write \"Player 1 wins\" end.; \n

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"rock\" then begin write \"Player 2 wins\" end.; \n

            write \"Do you want to play again?\"
            read REPLAY;

            end. \n
        end. \n
    end. \n
    ";

    let language = Args::parse().language;

    let stream = InputStream::new(test);
    let lexer = mini_imp::miniimplexer::MiniImpLexer::new(stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = MiniImpParser::new(token_stream);

    let mut previous_token: Option<Box<GenericToken<Cow<str>>>> = None;
    let mut output = String::new();
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

        let token = match language {
            Languages::Rust => handle_token(previous_token.clone(), current.clone(), next, &Rust),
        };

        output.push_str(&token);
        print!("{token}");
        previous_token = Some(current);
        if stream.la(1) != -1 {
            stream.consume();
        } else {
            parser.matched_eof = true;
        }
    }
    let file_type = match language {
        Languages::Rust => "rs",
    };
    fs::write(format!("output.{file_type}"), output).unwrap();
}

fn handle_token(
    previous: Option<Box<GenericToken<Cow<str>>>>,
    current: Box<GenericToken<Cow<str>>>,
    next: Box<GenericToken<Cow<str>>>,
    language: &impl TranslateMiniImpPlus,
) -> String {
    language.translate(
        MiniImpPlus::from(current),
        previous.map(MiniImpPlus::from),
        MiniImpPlus::from(next),
    )
}
