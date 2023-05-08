// Generated from MiniImp.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, EmptyContext, EmptyCustomRuleContext};
use antlr_rust::token::*;
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::{lazy_static, Tid, TidAble, TidExt};

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const T__7: isize = 8;
pub const T__8: isize = 9;
pub const T__9: isize = 10;
pub const T__10: isize = 11;
pub const T__11: isize = 12;
pub const T__12: isize = 13;
pub const T__13: isize = 14;
pub const T__14: isize = 15;
pub const T__15: isize = 16;
pub const T__16: isize = 17;
pub const T__17: isize = 18;
pub const T__18: isize = 19;
pub const T__19: isize = 20;
pub const T__20: isize = 21;
pub const T__21: isize = 22;
pub const Identifier: isize = 23;
pub const Number: isize = 24;
pub const WS: isize = 25;
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 27] = [
    "T__0",
    "T__1",
    "T__2",
    "T__3",
    "T__4",
    "T__5",
    "T__6",
    "T__7",
    "T__8",
    "T__9",
    "T__10",
    "T__11",
    "T__12",
    "T__13",
    "T__14",
    "T__15",
    "T__16",
    "T__17",
    "T__18",
    "T__19",
    "T__20",
    "T__21",
    "Digit",
    "Letter",
    "Identifier",
    "Number",
    "WS",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 23] = [
    None,
    Some("'true'"),
    Some("'false'"),
    Some("'not'"),
    Some("'is'"),
    Some("'+'"),
    Some("'-'"),
    Some("'*'"),
    Some("'/'"),
    Some("'('"),
    Some("')'"),
    Some("'if'"),
    Some("'then'"),
    Some("'else'"),
    Some("'while'"),
    Some("'set'"),
    Some("'='"),
    Some("';'"),
    Some("'write'"),
    Some("'var'"),
    Some("'begin'"),
    Some("'end.'"),
    Some("'program'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 26] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("Identifier"),
    Some("Number"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

pub type LexerContext<'input> =
    BaseRuleContext<'input, EmptyCustomRuleContext<'input, LocalTokenFactory<'input>>>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

pub struct MiniImpLexer<'input, Input: CharStream<From<'input>>> {
    base: BaseLexer<'input, MiniImpLexerActions, Input, LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for MiniImpLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input: CharStream<From<'input>>> Deref for MiniImpLexer<'input, Input> {
    type Target = BaseLexer<'input, MiniImpLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut for MiniImpLexer<'input, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> MiniImpLexer<'input, Input> {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "MiniImpLexer.g4"
    }

    pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                MiniImpLexerActions {},
                tf,
            ),
        }
    }
}

impl<'input, Input: CharStream<From<'input>>> MiniImpLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        MiniImpLexer::new_with_token_factory(
            input,
            <&LocalTokenFactory<'input> as Default>::default(),
        )
    }
}

pub struct MiniImpLexerActions {}

impl MiniImpLexerActions {}

impl<'input, Input: CharStream<From<'input>>>
    Actions<'input, BaseLexer<'input, MiniImpLexerActions, Input, LocalTokenFactory<'input>>>
    for MiniImpLexerActions
{
}

impl<'input, Input: CharStream<From<'input>>> MiniImpLexer<'input, Input> {}

impl<'input, Input: CharStream<From<'input>>>
    LexerRecog<'input, BaseLexer<'input, MiniImpLexerActions, Input, LocalTokenFactory<'input>>>
    for MiniImpLexerActions
{
}
impl<'input> TokenAware<'input> for MiniImpLexerActions {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input> for MiniImpLexer<'input, Input> {
    type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x1b\u{aa}\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\
		\x05\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\
		\x09\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\
		\x0e\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\
		\x12\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\
		\x17\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\
		\x1b\x04\x1c\x09\x1c\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\
		\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\
		\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\
		\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\
		\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
		\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\
		\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x03\
		\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\
		\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
		\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x18\x03\
		\x18\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x07\x1a\u{98}\x0a\
		\x1a\x0c\x1a\x0e\x1a\u{9b}\x0b\x1a\x03\x1b\x03\x1b\x07\x1b\u{9f}\x0a\x1b\
		\x0c\x1b\x0e\x1b\u{a2}\x0b\x1b\x03\x1c\x06\x1c\u{a5}\x0a\x1c\x0d\x1c\x0e\
		\x1c\u{a6}\x03\x1c\x03\x1c\x02\x02\x1d\x03\x03\x05\x04\x07\x05\x09\x06\
		\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\
		\x1d\x10\x1f\x11\x21\x12\x23\x13\x25\x14\x27\x15\x29\x16\x2b\x17\x2d\x18\
		\x2f\x02\x31\x02\x33\x19\x35\x1a\x37\x1b\x03\x02\x04\x03\x02\x43\x5c\x05\
		\x02\x0b\x0c\x0f\x0f\x22\x22\x02\u{ac}\x02\x03\x03\x02\x02\x02\x02\x05\
		\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\
		\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\
		\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\
		\x03\x02\x02\x02\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\
		\x03\x02\x02\x02\x02\x1f\x03\x02\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\
		\x03\x02\x02\x02\x02\x25\x03\x02\x02\x02\x02\x27\x03\x02\x02\x02\x02\x29\
		\x03\x02\x02\x02\x02\x2b\x03\x02\x02\x02\x02\x2d\x03\x02\x02\x02\x02\x33\
		\x03\x02\x02\x02\x02\x35\x03\x02\x02\x02\x02\x37\x03\x02\x02\x02\x03\x39\
		\x03\x02\x02\x02\x05\x3e\x03\x02\x02\x02\x07\x44\x03\x02\x02\x02\x09\x48\
		\x03\x02\x02\x02\x0b\x4b\x03\x02\x02\x02\x0d\x4d\x03\x02\x02\x02\x0f\x4f\
		\x03\x02\x02\x02\x11\x51\x03\x02\x02\x02\x13\x53\x03\x02\x02\x02\x15\x55\
		\x03\x02\x02\x02\x17\x57\x03\x02\x02\x02\x19\x5a\x03\x02\x02\x02\x1b\x5f\
		\x03\x02\x02\x02\x1d\x64\x03\x02\x02\x02\x1f\x6a\x03\x02\x02\x02\x21\x6e\
		\x03\x02\x02\x02\x23\x70\x03\x02\x02\x02\x25\x72\x03\x02\x02\x02\x27\x78\
		\x03\x02\x02\x02\x29\x7c\x03\x02\x02\x02\x2b\u{82}\x03\x02\x02\x02\x2d\
		\u{87}\x03\x02\x02\x02\x2f\u{8f}\x03\x02\x02\x02\x31\u{91}\x03\x02\x02\
		\x02\x33\u{93}\x03\x02\x02\x02\x35\u{9c}\x03\x02\x02\x02\x37\u{a4}\x03\
		\x02\x02\x02\x39\x3a\x07\x76\x02\x02\x3a\x3b\x07\x74\x02\x02\x3b\x3c\x07\
		\x77\x02\x02\x3c\x3d\x07\x67\x02\x02\x3d\x04\x03\x02\x02\x02\x3e\x3f\x07\
		\x68\x02\x02\x3f\x40\x07\x63\x02\x02\x40\x41\x07\x6e\x02\x02\x41\x42\x07\
		\x75\x02\x02\x42\x43\x07\x67\x02\x02\x43\x06\x03\x02\x02\x02\x44\x45\x07\
		\x70\x02\x02\x45\x46\x07\x71\x02\x02\x46\x47\x07\x76\x02\x02\x47\x08\x03\
		\x02\x02\x02\x48\x49\x07\x6b\x02\x02\x49\x4a\x07\x75\x02\x02\x4a\x0a\x03\
		\x02\x02\x02\x4b\x4c\x07\x2d\x02\x02\x4c\x0c\x03\x02\x02\x02\x4d\x4e\x07\
		\x2f\x02\x02\x4e\x0e\x03\x02\x02\x02\x4f\x50\x07\x2c\x02\x02\x50\x10\x03\
		\x02\x02\x02\x51\x52\x07\x31\x02\x02\x52\x12\x03\x02\x02\x02\x53\x54\x07\
		\x2a\x02\x02\x54\x14\x03\x02\x02\x02\x55\x56\x07\x2b\x02\x02\x56\x16\x03\
		\x02\x02\x02\x57\x58\x07\x6b\x02\x02\x58\x59\x07\x68\x02\x02\x59\x18\x03\
		\x02\x02\x02\x5a\x5b\x07\x76\x02\x02\x5b\x5c\x07\x6a\x02\x02\x5c\x5d\x07\
		\x67\x02\x02\x5d\x5e\x07\x70\x02\x02\x5e\x1a\x03\x02\x02\x02\x5f\x60\x07\
		\x67\x02\x02\x60\x61\x07\x6e\x02\x02\x61\x62\x07\x75\x02\x02\x62\x63\x07\
		\x67\x02\x02\x63\x1c\x03\x02\x02\x02\x64\x65\x07\x79\x02\x02\x65\x66\x07\
		\x6a\x02\x02\x66\x67\x07\x6b\x02\x02\x67\x68\x07\x6e\x02\x02\x68\x69\x07\
		\x67\x02\x02\x69\x1e\x03\x02\x02\x02\x6a\x6b\x07\x75\x02\x02\x6b\x6c\x07\
		\x67\x02\x02\x6c\x6d\x07\x76\x02\x02\x6d\x20\x03\x02\x02\x02\x6e\x6f\x07\
		\x3f\x02\x02\x6f\x22\x03\x02\x02\x02\x70\x71\x07\x3d\x02\x02\x71\x24\x03\
		\x02\x02\x02\x72\x73\x07\x79\x02\x02\x73\x74\x07\x74\x02\x02\x74\x75\x07\
		\x6b\x02\x02\x75\x76\x07\x76\x02\x02\x76\x77\x07\x67\x02\x02\x77\x26\x03\
		\x02\x02\x02\x78\x79\x07\x78\x02\x02\x79\x7a\x07\x63\x02\x02\x7a\x7b\x07\
		\x74\x02\x02\x7b\x28\x03\x02\x02\x02\x7c\x7d\x07\x64\x02\x02\x7d\x7e\x07\
		\x67\x02\x02\x7e\x7f\x07\x69\x02\x02\x7f\u{80}\x07\x6b\x02\x02\u{80}\u{81}\
		\x07\x70\x02\x02\u{81}\x2a\x03\x02\x02\x02\u{82}\u{83}\x07\x67\x02\x02\
		\u{83}\u{84}\x07\x70\x02\x02\u{84}\u{85}\x07\x66\x02\x02\u{85}\u{86}\x07\
		\x30\x02\x02\u{86}\x2c\x03\x02\x02\x02\u{87}\u{88}\x07\x72\x02\x02\u{88}\
		\u{89}\x07\x74\x02\x02\u{89}\u{8a}\x07\x71\x02\x02\u{8a}\u{8b}\x07\x69\
		\x02\x02\u{8b}\u{8c}\x07\x74\x02\x02\u{8c}\u{8d}\x07\x63\x02\x02\u{8d}\
		\u{8e}\x07\x6f\x02\x02\u{8e}\x2e\x03\x02\x02\x02\u{8f}\u{90}\x04\x32\x3b\
		\x02\u{90}\x30\x03\x02\x02\x02\u{91}\u{92}\x09\x02\x02\x02\u{92}\x32\x03\
		\x02\x02\x02\u{93}\u{99}\x05\x31\x19\x02\u{94}\u{98}\x05\x31\x19\x02\u{95}\
		\u{98}\x05\x2f\x18\x02\u{96}\u{98}\x07\x61\x02\x02\u{97}\u{94}\x03\x02\
		\x02\x02\u{97}\u{95}\x03\x02\x02\x02\u{97}\u{96}\x03\x02\x02\x02\u{98}\
		\u{9b}\x03\x02\x02\x02\u{99}\u{97}\x03\x02\x02\x02\u{99}\u{9a}\x03\x02\
		\x02\x02\u{9a}\x34\x03\x02\x02\x02\u{9b}\u{99}\x03\x02\x02\x02\u{9c}\u{a0}\
		\x05\x2f\x18\x02\u{9d}\u{9f}\x05\x2f\x18\x02\u{9e}\u{9d}\x03\x02\x02\x02\
		\u{9f}\u{a2}\x03\x02\x02\x02\u{a0}\u{9e}\x03\x02\x02\x02\u{a0}\u{a1}\x03\
		\x02\x02\x02\u{a1}\x36\x03\x02\x02\x02\u{a2}\u{a0}\x03\x02\x02\x02\u{a3}\
		\u{a5}\x09\x03\x02\x02\u{a4}\u{a3}\x03\x02\x02\x02\u{a5}\u{a6}\x03\x02\
		\x02\x02\u{a6}\u{a4}\x03\x02\x02\x02\u{a6}\u{a7}\x03\x02\x02\x02\u{a7}\
		\u{a8}\x03\x02\x02\x02\u{a8}\u{a9}\x08\x1c\x02\x02\u{a9}\x38\x03\x02\x02\
		\x02\x07\x02\u{97}\u{99}\u{a0}\u{a6}\x03\x08\x02\x02";
