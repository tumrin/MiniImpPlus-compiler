// Generated from MiniImp.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const T__2:isize=3; 
	pub const T__3:isize=4; 
	pub const T__4:isize=5; 
	pub const T__5:isize=6; 
	pub const T__6:isize=7; 
	pub const T__7:isize=8; 
	pub const T__8:isize=9; 
	pub const T__9:isize=10; 
	pub const T__10:isize=11; 
	pub const T__11:isize=12; 
	pub const T__12:isize=13; 
	pub const T__13:isize=14; 
	pub const T__14:isize=15; 
	pub const T__15:isize=16; 
	pub const T__16:isize=17; 
	pub const T__17:isize=18; 
	pub const T__18:isize=19; 
	pub const T__19:isize=20; 
	pub const T__20:isize=21; 
	pub const T__21:isize=22; 
	pub const T__22:isize=23; 
	pub const T__23:isize=24; 
	pub const T__24:isize=25; 
	pub const T__25:isize=26; 
	pub const T__26:isize=27; 
	pub const Identifier:isize=28; 
	pub const Number:isize=29; 
	pub const WS:isize=30; 
	pub const STRING:isize=31;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;33] = [
		"T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
		"T__9", "T__10", "T__11", "T__12", "T__13", "T__14", "T__15", "T__16", 
		"T__17", "T__18", "T__19", "T__20", "T__21", "T__22", "T__23", "T__24", 
		"T__25", "T__26", "Digit", "Letter", "Identifier", "Number", "WS", "STRING"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;28] = [
		None, Some("'true'"), Some("'false'"), Some("'not'"), Some("'is'"), Some("'or'"), 
		Some("'and'"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'('"), 
		Some("')'"), Some("'if'"), Some("'then'"), Some("'else'"), Some("'while'"), 
		Some("'set'"), Some("'='"), Some("';'"), Some("'write'"), Some("'read'"), 
		Some("'var'"), Some("'asNumber'"), Some("'asString'"), Some("'begin'"), 
		Some("'end.'"), Some("'program'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;32]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, Some("Identifier"), Some("Number"), Some("WS"), 
		Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct MiniImpLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,MiniImpLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for MiniImpLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for MiniImpLexer<'input,Input>{
	type Target = BaseLexer<'input,MiniImpLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for MiniImpLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> MiniImpLexer<'input,Input>{
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
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				MiniImpLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> MiniImpLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		MiniImpLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct MiniImpLexerActions {
}

impl MiniImpLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,MiniImpLexerActions,Input,LocalTokenFactory<'input>>> for MiniImpLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> MiniImpLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,MiniImpLexerActions,Input,LocalTokenFactory<'input>>> for MiniImpLexerActions{
}
impl<'input> TokenAware<'input> for MiniImpLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for MiniImpLexer<'input,Input>{
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
	            dfa.push(DFA::new(
	                _ATN.clone(),
	                _ATN.get_decision_state(i),
	                i as isize,
	            ).into())
	        }
	        Arc::new(dfa)
	    };
	}



	const _serializedATN:&'static str =
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x21\u{dd}\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\
		\x05\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\
		\x09\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\
		\x0e\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\
		\x12\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\
		\x17\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\
		\x1b\x04\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\
		\x20\x09\x20\x04\x21\x09\x21\x04\x22\x09\x22\x03\x02\x03\x02\x03\x02\x03\
		\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\
		\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\
		\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\x03\x0a\x03\
		\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\
		\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\
		\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x12\x03\
		\x12\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\x03\x14\x03\x15\x03\x15\x03\
		\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
		\x17\x03\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\
		\x18\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
		\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
		\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1c\x03\
		\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\
		\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{c2}\x0a\x1f\x0c\x1f\x0e\x1f\u{c5}\
		\x0b\x1f\x03\x20\x03\x20\x07\x20\u{c9}\x0a\x20\x0c\x20\x0e\x20\u{cc}\x0b\
		\x20\x03\x21\x06\x21\u{cf}\x0a\x21\x0d\x21\x0e\x21\u{d0}\x03\x21\x03\x21\
		\x03\x22\x03\x22\x07\x22\u{d7}\x0a\x22\x0c\x22\x0e\x22\u{da}\x0b\x22\x03\
		\x22\x03\x22\x02\x02\x23\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\
		\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\x1d\x10\x1f\x11\
		\x21\x12\x23\x13\x25\x14\x27\x15\x29\x16\x2b\x17\x2d\x18\x2f\x19\x31\x1a\
		\x33\x1b\x35\x1c\x37\x1d\x39\x02\x3b\x02\x3d\x1e\x3f\x1f\x41\x20\x43\x21\
		\x03\x02\x05\x03\x02\x43\x5c\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x03\x02\x24\
		\x24\x02\u{e0}\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\
		\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\
		\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\
		\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\x02\x02\x19\
		\x03\x02\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\x03\x02\x02\x02\x02\x1f\
		\x03\x02\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\x03\x02\x02\x02\x02\x25\
		\x03\x02\x02\x02\x02\x27\x03\x02\x02\x02\x02\x29\x03\x02\x02\x02\x02\x2b\
		\x03\x02\x02\x02\x02\x2d\x03\x02\x02\x02\x02\x2f\x03\x02\x02\x02\x02\x31\
		\x03\x02\x02\x02\x02\x33\x03\x02\x02\x02\x02\x35\x03\x02\x02\x02\x02\x37\
		\x03\x02\x02\x02\x02\x3d\x03\x02\x02\x02\x02\x3f\x03\x02\x02\x02\x02\x41\
		\x03\x02\x02\x02\x02\x43\x03\x02\x02\x02\x03\x45\x03\x02\x02\x02\x05\x4a\
		\x03\x02\x02\x02\x07\x50\x03\x02\x02\x02\x09\x54\x03\x02\x02\x02\x0b\x57\
		\x03\x02\x02\x02\x0d\x5a\x03\x02\x02\x02\x0f\x5e\x03\x02\x02\x02\x11\x60\
		\x03\x02\x02\x02\x13\x62\x03\x02\x02\x02\x15\x64\x03\x02\x02\x02\x17\x66\
		\x03\x02\x02\x02\x19\x68\x03\x02\x02\x02\x1b\x6a\x03\x02\x02\x02\x1d\x6d\
		\x03\x02\x02\x02\x1f\x72\x03\x02\x02\x02\x21\x77\x03\x02\x02\x02\x23\x7d\
		\x03\x02\x02\x02\x25\u{81}\x03\x02\x02\x02\x27\u{83}\x03\x02\x02\x02\x29\
		\u{85}\x03\x02\x02\x02\x2b\u{8b}\x03\x02\x02\x02\x2d\u{90}\x03\x02\x02\
		\x02\x2f\u{94}\x03\x02\x02\x02\x31\u{9d}\x03\x02\x02\x02\x33\u{a6}\x03\
		\x02\x02\x02\x35\u{ac}\x03\x02\x02\x02\x37\u{b1}\x03\x02\x02\x02\x39\u{b9}\
		\x03\x02\x02\x02\x3b\u{bb}\x03\x02\x02\x02\x3d\u{bd}\x03\x02\x02\x02\x3f\
		\u{c6}\x03\x02\x02\x02\x41\u{ce}\x03\x02\x02\x02\x43\u{d4}\x03\x02\x02\
		\x02\x45\x46\x07\x76\x02\x02\x46\x47\x07\x74\x02\x02\x47\x48\x07\x77\x02\
		\x02\x48\x49\x07\x67\x02\x02\x49\x04\x03\x02\x02\x02\x4a\x4b\x07\x68\x02\
		\x02\x4b\x4c\x07\x63\x02\x02\x4c\x4d\x07\x6e\x02\x02\x4d\x4e\x07\x75\x02\
		\x02\x4e\x4f\x07\x67\x02\x02\x4f\x06\x03\x02\x02\x02\x50\x51\x07\x70\x02\
		\x02\x51\x52\x07\x71\x02\x02\x52\x53\x07\x76\x02\x02\x53\x08\x03\x02\x02\
		\x02\x54\x55\x07\x6b\x02\x02\x55\x56\x07\x75\x02\x02\x56\x0a\x03\x02\x02\
		\x02\x57\x58\x07\x71\x02\x02\x58\x59\x07\x74\x02\x02\x59\x0c\x03\x02\x02\
		\x02\x5a\x5b\x07\x63\x02\x02\x5b\x5c\x07\x70\x02\x02\x5c\x5d\x07\x66\x02\
		\x02\x5d\x0e\x03\x02\x02\x02\x5e\x5f\x07\x2d\x02\x02\x5f\x10\x03\x02\x02\
		\x02\x60\x61\x07\x2f\x02\x02\x61\x12\x03\x02\x02\x02\x62\x63\x07\x2c\x02\
		\x02\x63\x14\x03\x02\x02\x02\x64\x65\x07\x31\x02\x02\x65\x16\x03\x02\x02\
		\x02\x66\x67\x07\x2a\x02\x02\x67\x18\x03\x02\x02\x02\x68\x69\x07\x2b\x02\
		\x02\x69\x1a\x03\x02\x02\x02\x6a\x6b\x07\x6b\x02\x02\x6b\x6c\x07\x68\x02\
		\x02\x6c\x1c\x03\x02\x02\x02\x6d\x6e\x07\x76\x02\x02\x6e\x6f\x07\x6a\x02\
		\x02\x6f\x70\x07\x67\x02\x02\x70\x71\x07\x70\x02\x02\x71\x1e\x03\x02\x02\
		\x02\x72\x73\x07\x67\x02\x02\x73\x74\x07\x6e\x02\x02\x74\x75\x07\x75\x02\
		\x02\x75\x76\x07\x67\x02\x02\x76\x20\x03\x02\x02\x02\x77\x78\x07\x79\x02\
		\x02\x78\x79\x07\x6a\x02\x02\x79\x7a\x07\x6b\x02\x02\x7a\x7b\x07\x6e\x02\
		\x02\x7b\x7c\x07\x67\x02\x02\x7c\x22\x03\x02\x02\x02\x7d\x7e\x07\x75\x02\
		\x02\x7e\x7f\x07\x67\x02\x02\x7f\u{80}\x07\x76\x02\x02\u{80}\x24\x03\x02\
		\x02\x02\u{81}\u{82}\x07\x3f\x02\x02\u{82}\x26\x03\x02\x02\x02\u{83}\u{84}\
		\x07\x3d\x02\x02\u{84}\x28\x03\x02\x02\x02\u{85}\u{86}\x07\x79\x02\x02\
		\u{86}\u{87}\x07\x74\x02\x02\u{87}\u{88}\x07\x6b\x02\x02\u{88}\u{89}\x07\
		\x76\x02\x02\u{89}\u{8a}\x07\x67\x02\x02\u{8a}\x2a\x03\x02\x02\x02\u{8b}\
		\u{8c}\x07\x74\x02\x02\u{8c}\u{8d}\x07\x67\x02\x02\u{8d}\u{8e}\x07\x63\
		\x02\x02\u{8e}\u{8f}\x07\x66\x02\x02\u{8f}\x2c\x03\x02\x02\x02\u{90}\u{91}\
		\x07\x78\x02\x02\u{91}\u{92}\x07\x63\x02\x02\u{92}\u{93}\x07\x74\x02\x02\
		\u{93}\x2e\x03\x02\x02\x02\u{94}\u{95}\x07\x63\x02\x02\u{95}\u{96}\x07\
		\x75\x02\x02\u{96}\u{97}\x07\x50\x02\x02\u{97}\u{98}\x07\x77\x02\x02\u{98}\
		\u{99}\x07\x6f\x02\x02\u{99}\u{9a}\x07\x64\x02\x02\u{9a}\u{9b}\x07\x67\
		\x02\x02\u{9b}\u{9c}\x07\x74\x02\x02\u{9c}\x30\x03\x02\x02\x02\u{9d}\u{9e}\
		\x07\x63\x02\x02\u{9e}\u{9f}\x07\x75\x02\x02\u{9f}\u{a0}\x07\x55\x02\x02\
		\u{a0}\u{a1}\x07\x76\x02\x02\u{a1}\u{a2}\x07\x74\x02\x02\u{a2}\u{a3}\x07\
		\x6b\x02\x02\u{a3}\u{a4}\x07\x70\x02\x02\u{a4}\u{a5}\x07\x69\x02\x02\u{a5}\
		\x32\x03\x02\x02\x02\u{a6}\u{a7}\x07\x64\x02\x02\u{a7}\u{a8}\x07\x67\x02\
		\x02\u{a8}\u{a9}\x07\x69\x02\x02\u{a9}\u{aa}\x07\x6b\x02\x02\u{aa}\u{ab}\
		\x07\x70\x02\x02\u{ab}\x34\x03\x02\x02\x02\u{ac}\u{ad}\x07\x67\x02\x02\
		\u{ad}\u{ae}\x07\x70\x02\x02\u{ae}\u{af}\x07\x66\x02\x02\u{af}\u{b0}\x07\
		\x30\x02\x02\u{b0}\x36\x03\x02\x02\x02\u{b1}\u{b2}\x07\x72\x02\x02\u{b2}\
		\u{b3}\x07\x74\x02\x02\u{b3}\u{b4}\x07\x71\x02\x02\u{b4}\u{b5}\x07\x69\
		\x02\x02\u{b5}\u{b6}\x07\x74\x02\x02\u{b6}\u{b7}\x07\x63\x02\x02\u{b7}\
		\u{b8}\x07\x6f\x02\x02\u{b8}\x38\x03\x02\x02\x02\u{b9}\u{ba}\x04\x32\x3b\
		\x02\u{ba}\x3a\x03\x02\x02\x02\u{bb}\u{bc}\x09\x02\x02\x02\u{bc}\x3c\x03\
		\x02\x02\x02\u{bd}\u{c3}\x05\x3b\x1e\x02\u{be}\u{c2}\x05\x3b\x1e\x02\u{bf}\
		\u{c2}\x05\x39\x1d\x02\u{c0}\u{c2}\x07\x61\x02\x02\u{c1}\u{be}\x03\x02\
		\x02\x02\u{c1}\u{bf}\x03\x02\x02\x02\u{c1}\u{c0}\x03\x02\x02\x02\u{c2}\
		\u{c5}\x03\x02\x02\x02\u{c3}\u{c1}\x03\x02\x02\x02\u{c3}\u{c4}\x03\x02\
		\x02\x02\u{c4}\x3e\x03\x02\x02\x02\u{c5}\u{c3}\x03\x02\x02\x02\u{c6}\u{ca}\
		\x05\x39\x1d\x02\u{c7}\u{c9}\x05\x39\x1d\x02\u{c8}\u{c7}\x03\x02\x02\x02\
		\u{c9}\u{cc}\x03\x02\x02\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{cb}\x03\
		\x02\x02\x02\u{cb}\x40\x03\x02\x02\x02\u{cc}\u{ca}\x03\x02\x02\x02\u{cd}\
		\u{cf}\x09\x03\x02\x02\u{ce}\u{cd}\x03\x02\x02\x02\u{cf}\u{d0}\x03\x02\
		\x02\x02\u{d0}\u{ce}\x03\x02\x02\x02\u{d0}\u{d1}\x03\x02\x02\x02\u{d1}\
		\u{d2}\x03\x02\x02\x02\u{d2}\u{d3}\x08\x21\x02\x02\u{d3}\x42\x03\x02\x02\
		\x02\u{d4}\u{d8}\x07\x24\x02\x02\u{d5}\u{d7}\x0a\x04\x02\x02\u{d6}\u{d5}\
		\x03\x02\x02\x02\u{d7}\u{da}\x03\x02\x02\x02\u{d8}\u{d6}\x03\x02\x02\x02\
		\u{d8}\u{d9}\x03\x02\x02\x02\u{d9}\u{db}\x03\x02\x02\x02\u{da}\u{d8}\x03\
		\x02\x02\x02\u{db}\u{dc}\x07\x24\x02\x02\u{dc}\x44\x03\x02\x02\x02\x08\
		\x02\u{c1}\u{c3}\u{ca}\u{d0}\u{d8}\x03\x08\x02\x02";
