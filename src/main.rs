use antlr_rust::{
    common_token_stream::CommonTokenStream,
    token::{GenericToken, Token},
    InputStream, Parser,
};
use clap::Parser as ArgParser;
use mini_imp::miniimpparser::MiniImpParser;
use mini_imp_plus::{
    languages::{js::Javascript, rust::Rust, Languages},
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
    let test =
        fs::read_to_string("testprogram.mip").expect("Expected testprogram.mip in project root");

    // The language we are translating to
    let language = Args::parse().language;

    let stream = InputStream::new(test.as_str());
    let lexer = mini_imp::miniimplexer::MiniImpLexer::new(stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = MiniImpParser::new(token_stream);

    let mut previous_token: Option<Box<GenericToken<Cow<str>>>> = None;

    // To-be contents of the source code of the destination language
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

        // Translate the token to the destination language
        let token = match language {
            Languages::Rust => handle_token(previous_token.clone(), current.clone(), next, &Rust),
            Languages::Javascript => {
                handle_token(previous_token.clone(), current.clone(), next, &Javascript)
            }
        };

        // Add evaluated token to output
        output.push_str(&token);
        print!("{token}");
        previous_token = Some(current);
        if stream.la(1) != -1 {
            stream.consume();
        } else {
            parser.matched_eof = true;
        }
    }

    // Choose the output file type according to the destination language
    let file_type = match language {
        Languages::Rust => "rs",
        Languages::Javascript => "mjs",
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
