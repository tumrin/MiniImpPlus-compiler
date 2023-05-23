use antlr_rust::{
    common_token_stream::CommonTokenStream,
    parser_rule_context::ParserRuleContext,
    rule_context::CustomRuleContext,
    token::{GenericToken, Token},
    tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree},
    InputStream, Parser,
};
use clap::Parser as ArgParser;
use mini_imp::{
    miniimpparser::{DeclContextAttrs, MiniImpParser, MiniImpParserContextType},
    miniimpvisitor::{MiniImpVisitor, MiniImpVisitorCompat},
};
use mini_imp_plus::{
    languages::{js::Javascript, rust::Rust, Languages},
    MiniImpPlus, TranslateMiniImpPlus,
};
use std::{borrow::Cow, format, fs};

use crate::mini_imp::miniimpparser::{
    DeclsContextAttrs, ExprContextAttrs, FactorContextAttrs, ProgContextAttrs, ScopeContextAttrs,
    SetContextAttrs, StmtsContextAttrs, TermContextAttrs, TruthContextAttrs, VariableContextAttrs,
};

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
    "program DEMOAPP 
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

            write \"Choose a gesture (rock/paper/scissors) for the following player: \";
            write PLAYER_ONE;
            read PLAYER_ONE_GESTURE; 

            write \"Choose a gesture (rock/paper/scissors) for the following player: \";
            write PLAYER_TWO;
            read PLAYER_TWO_GESTURE; 

            if is PLAYER_TWO_GESTURE PLAYER_ONE_GESTURE then begin write \"Draw\"; 
            
            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write PLAYER_TWO; write \" wins\"; 

            if is PLAYER_TWO_GESTURE \"rock\" and is PLAYER_ONE_GESTURE \"paper\" then begin write PLAYER_ONE; write \" wins\" ; 

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"paper\" then begin write PLAYER_TWO; write \" wins\" ; 

            if is PLAYER_TWO_GESTURE \"scissors\" and is PLAYER_ONE_GESTURE \"rock\" then begin write PLAYER_ONE; write \" wins\" ; 

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"scissors\" then begin write PLAYER_ONE; write \" wins\" ; 

            if is PLAYER_TWO_GESTURE \"paper\" and is PLAYER_ONE_GESTURE \"rock\" then begin write PLAYER_TWO; write \" wins\"; 

            write \"Do you want to play again?\";
            read INPUT;
            if is INPUT \"yes\" then begin set REPLAY = true; end. else begin set REPLAY = false;
        end.
    end.;";

    let language = Args::parse().language;

    let stream = InputStream::new(test);
    let lexer = mini_imp::miniimplexer::MiniImpLexer::new(stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = MiniImpParser::new(token_stream);

    let mut previous_token: Option<Box<GenericToken<Cow<str>>>> = None;
    let mut output = String::new();

    let root = parser.prog().unwrap();
    let mut visitor = TestVisitor(String::new());
    let output = visitor.visit(&*root);
    println!("{output}");
    // while !parser.matched_eof {
    //     let current = parser.get_current_token().clone();
    //     let stream = parser.get_input_stream_mut();
    //
    //     // if next is not last add 1 to index, yes this is hacky
    //     let next_index = if stream.la(2) != -1 {
    //         current.get_token_index() + 1
    //     } else {
    //         current.get_token_index()
    //     };
    //     let next = stream.get(next_index).clone();
    //
    //     let token = match language {
    //         Languages::Rust => handle_token(previous_token.clone(), current.clone(), next, &Rust),
    //         Languages::Javascript => {
    //             handle_token(previous_token.clone(), current.clone(), next, &Javascript)
    //         }
    //     };
    //
    //     output.push_str(&token);
    //     previous_token = Some(current);
    //     if stream.la(1) != -1 {
    //         stream.consume();
    //     } else {
    //         parser.matched_eof = true;
    //     }
    // }
    // let file_type = match language {
    //     Languages::Rust => "rs",
    //     Languages::Javascript => "mjs",
    // };
    fs::write(format!("output.rs"), output).unwrap();
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

struct TestVisitor(String);

impl ParseTreeVisitorCompat<'_> for TestVisitor {
    type Node = MiniImpParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        _node.symbol.text.to_string()
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }
}
impl MiniImpVisitorCompat<'_> for TestVisitor {
    fn visit_truth(&mut self, ctx: &mini_imp::miniimpparser::TruthContext<'_>) -> Self::Return {
        if ctx.Identifier().is_some() {
            print!("truth{:?}", ctx.Identifier().unwrap());
        } else if ctx.expr().is_some() {
            print!("{:?}", ctx.expr().unwrap());
        }
        self.visit_children(ctx)
    }

    fn visit_expr(&mut self, ctx: &mini_imp::miniimpparser::ExprContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_term(&mut self, ctx: &mini_imp::miniimpparser::TermContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_factor(&mut self, ctx: &mini_imp::miniimpparser::FactorContext<'_>) -> Self::Return {
        if ctx.STRING().is_some() {
            print!("{:?}", ctx.STRING().unwrap());
        } else if ctx.truth().is_some() {
            print!("{:?}", ctx.truth().unwrap().get_text());
        } else if ctx.Identifier().is_some() {
            print!("{:?}", ctx.Identifier().unwrap());
        } else if ctx.expr().is_some() {
            print!("{:?}", ctx.expr());
        }
        self.visit_children(ctx)
    }

    fn visit_stmt(&mut self, ctx: &mini_imp::miniimpparser::StmtContext<'_>) -> Self::Return {
        println!("");
        self.visit_children(ctx)
    }

    fn visit_select(&mut self, ctx: &mini_imp::miniimpparser::SelectContext<'_>) -> Self::Return {
        print!("if ");
        self.visit_children(ctx)
    }

    fn visit_iterat(&mut self, ctx: &mini_imp::miniimpparser::IteratContext<'_>) -> Self::Return {
        print!("while ");
        self.visit_children(ctx)
    }

    fn visit_set(&mut self, ctx: &mini_imp::miniimpparser::SetContext<'_>) -> Self::Return {
        print!("{:?} =", ctx.Identifier().unwrap());
        self.visit_children(ctx)
    }

    fn visit_write(&mut self, ctx: &mini_imp::miniimpparser::WriteContext<'_>) -> Self::Return {
        print!("println!(");
        let string = self.visit_children(ctx);
        print!(")");
        string
    }

    fn visit_read(&mut self, ctx: &mini_imp::miniimpparser::ReadContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_decl(&mut self, ctx: &mini_imp::miniimpparser::DeclContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_variable(
        &mut self,
        ctx: &mini_imp::miniimpparser::VariableContext<'_>,
    ) -> Self::Return {
        print!("let mut {:?} =", ctx.Identifier().unwrap());
        self.visit_children(ctx)
    }

    fn visit_asNumber(
        &mut self,
        ctx: &mini_imp::miniimpparser::AsNumberContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_asString(
        &mut self,
        ctx: &mini_imp::miniimpparser::AsStringContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_stmts(&mut self, ctx: &mini_imp::miniimpparser::StmtsContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_decls(&mut self, ctx: &mini_imp::miniimpparser::DeclsContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_scope(&mut self, ctx: &mini_imp::miniimpparser::ScopeContext<'_>) -> Self::Return {
        print!("{{\n");
        let string = self.visit_children(ctx);
        print!("}}\n");
        string
    }

    fn visit_init(&mut self, ctx: &mini_imp::miniimpparser::InitContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_prog(&mut self, ctx: &mini_imp::miniimpparser::ProgContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }
}
