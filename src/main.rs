use antlr_rust::{
    common_token_stream::CommonTokenStream, tree::ParseTreeVisitorCompat, InputStream,
};
use clap::Parser as ArgParser;
use mini_imp_plus::{
    languages::{js::JSVisitor, rust::RustVisitor, Languages},
    mini_imp::{miniimplexer::MiniImpLexer, miniimpparser::MiniImpParser},
};
use std::{format, fs};

/// Translates form MiniImp to an another language
#[derive(ArgParser, Debug)]
struct Args {
    /// Language to translate to
    #[clap(value_enum)]
    language: Languages,
}

fn main() {
    let test =
        fs::read_to_string("testprogram.mip").expect("expected testprogram.mip in project root");

    // Parse command line argument and get the language enum value
    let language = Args::parse().language;

    // Construct token stream and parser from the testprogram
    let stream = InputStream::new(test.as_str());
    let lexer = MiniImpLexer::new(stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = MiniImpParser::new(token_stream);

    let root = parser.prog().unwrap();

    // Get the output string and filename based on languages enum
    let (output, file) = match language {
        Languages::Rust => {
            let mut visitor = RustVisitor(String::new());
            (visitor.visit(&*root), "output.rs")
        }
        Languages::Javascript => {
            let mut visitor = JSVisitor(String::new());
            (visitor.visit(&*root), "output.mjs")
        }
    };
    fs::write(file, output).unwrap();
}
