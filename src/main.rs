use antlr_rust::{
    common_token_stream::CommonTokenStream,
    token::{GenericToken, Token},
    InputStream, Parser,
};
use clap::Parser as ArgParser;
use mini_imp::miniimpparser::MiniImpParser;
use mini_imp_plus::{
    languages::{rust::Rust, ts::Typescript, Languages},
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
    let test = "program DEMOAPP 
    begin 
        var REPLAY = true; 
        while REPLAY begin
            var PLAYER_ONE = \"\";
            var PLAYER_ONE_GESTURE = \"\"; 
            var PLAYER_TWO = \"\"; 
            var PLAYER_TWO_GESTURE = \"\";
            var INPUT = \"\";

            write \"Insert name of player 1:\";
            read PLAYER_ONE; 
            write \"Insert name of player 2:\";
            read PLAYER_TWO; 

            write \"Starting game with players:\";
            write PLAYER_ONE;
            write PLAYER_TWO;

            write \"Player 1, choose a gesture (rock/paper/scissors): \";
            read PLAYER_ONE_GESTURE; 

            write \"Player 2, choose a gesture (rock/paper/scissors): \"; 
            read PLAYER_TWO_GESTURE; 

            if is PLAYER_TWO_GESTURE PLAYER_ONE_GESTURE then begin write \"Draw\" end.; 
            
            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write PLAYER_TWO; write \" wins\" end.; 

            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"paper\" then begin write PLAYER_ONE; write \" wins\" end.; 

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"paper\" then begin write PLAYER_TWO; write \" wins\" end.; 

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"rock\" then begin write PLAYER_ONE; write \" wins\" end.; 

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write PLAYER_ONE; write \" wins\" end.; 

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"rock\" then begin write PLAYER_TWO; write \" wins\" end.; 

            write \"Do you want to play again?\";
            read INPUT;
            if is INPUT \"yes\" then begin set REPLAY = true end. else begin set REPLAY = false end.;
        end.
    end.";

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
            Languages::Typescript => {
                handle_token(previous_token.clone(), current.clone(), next, &Typescript)
            }
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
        Languages::Typescript => "ts",
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
