// Generated from MiniImp.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::miniimplistener::*;
use super::miniimpvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

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
	pub const RULE_truth:usize = 0; 
	pub const RULE_expr:usize = 1; 
	pub const RULE_term:usize = 2; 
	pub const RULE_factor:usize = 3; 
	pub const RULE_stmt:usize = 4; 
	pub const RULE_select:usize = 5; 
	pub const RULE_iterat:usize = 6; 
	pub const RULE_set:usize = 7; 
	pub const RULE_write:usize = 8; 
	pub const RULE_read:usize = 9; 
	pub const RULE_decl:usize = 10; 
	pub const RULE_variable:usize = 11; 
	pub const RULE_asNumber:usize = 12; 
	pub const RULE_asString:usize = 13; 
	pub const RULE_stmts:usize = 14; 
	pub const RULE_decls:usize = 15; 
	pub const RULE_scope:usize = 16; 
	pub const RULE_init:usize = 17; 
	pub const RULE_prog:usize = 18;
	pub const ruleNames: [&'static str; 19] =  [
		"truth", "expr", "term", "factor", "stmt", "select", "iterat", "set", 
		"write", "read", "decl", "variable", "asNumber", "asString", "stmts", 
		"decls", "scope", "init", "prog"
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


type BaseParserType<'input, I> =
	BaseParser<'input,MiniImpParserExt<'input>, I, MiniImpParserContextType , dyn MiniImpListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type MiniImpTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, MiniImpParserContextType , dyn MiniImpListener<'input> + 'a>;

/// Parser for MiniImp grammar
pub struct MiniImpParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				MiniImpParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> MiniImpParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> MiniImpParser<'input, I, DefaultErrorStrategy<'input,MiniImpParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for MiniImpParser
pub trait MiniImpParserContext<'input>:
	for<'x> Listenable<dyn MiniImpListener<'input> + 'x > + 
	for<'x> Visitable<dyn MiniImpVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=MiniImpParserContextType>
{}

antlr_rust::coerce_from!{ 'input : MiniImpParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn MiniImpParserContext<'input> + 'input
where
    T: MiniImpVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn MiniImpVisitor<'input> + 'x))
    }
}

impl<'input> MiniImpParserContext<'input> for TerminalNode<'input,MiniImpParserContextType> {}
impl<'input> MiniImpParserContext<'input> for ErrorNode<'input,MiniImpParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn MiniImpParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn MiniImpListener<'input> + 'input }

pub struct MiniImpParserContextType;
antlr_rust::tid!{MiniImpParserContextType}

impl<'input> ParserNodeType<'input> for MiniImpParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn MiniImpParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct MiniImpParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> MiniImpParserExt<'input>{
}
antlr_rust::tid! { MiniImpParserExt<'a> }

impl<'input> TokenAware<'input> for MiniImpParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for MiniImpParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for MiniImpParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "MiniImp.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn MiniImpParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					0 => MiniImpParser::<'input,I,_>::truth_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> MiniImpParser<'input, I, DefaultErrorStrategy<'input,MiniImpParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn truth_sempred(_localctx: Option<&TruthContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- truth ----------------
pub type TruthContextAll<'input> = TruthContext<'input>;


pub type TruthContext<'input> = BaseParserRuleContext<'input,TruthContextExt<'input>>;

#[derive(Clone)]
pub struct TruthContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for TruthContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for TruthContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_truth(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_truth(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for TruthContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_truth(self);
	}
}

impl<'input> CustomRuleContext<'input> for TruthContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_truth }
	//fn type_rule_index() -> usize where Self: Sized { RULE_truth }
}
antlr_rust::tid!{TruthContextExt<'a>}

impl<'input> TruthContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TruthContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TruthContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TruthContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<TruthContextExt<'input>>{

fn truth_all(&self) ->  Vec<Rc<TruthContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn truth(&self, i: usize) -> Option<Rc<TruthContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TruthContextAttrs<'input> for TruthContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  truth(&mut self,)
	-> Result<Rc<TruthContextAll<'input>>,ANTLRError> {
		self.truth_rec(0)
	}

	fn truth_rec(&mut self, _p: isize)
	-> Result<Rc<TruthContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = TruthContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 0, RULE_truth, _p);
	    let mut _localctx: Rc<TruthContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 0;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(46);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					{
					recog.base.set_state(39);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 T__1 
				=> {
					{
					recog.base.set_state(40);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__2 
				=> {
					{
					recog.base.set_state(41);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule truth*/
					recog.base.set_state(42);
					recog.truth_rec(4)?;

					}
				}

			 T__3 
				=> {
					{
					recog.base.set_state(43);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(44);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(45);
					recog.expr()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(56);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(54);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = TruthContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_truth);
							_localctx = tmp;
							recog.base.set_state(48);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(49);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							/*InvokeRule truth*/
							recog.base.set_state(50);
							recog.truth_rec(3)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = TruthContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_truth);
							_localctx = tmp;
							recog.base.set_state(51);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(52);
							recog.base.match_token(T__5,&mut recog.err_handler)?;

							/*InvokeRule truth*/
							recog.base.set_state(53);
							recog.truth_rec(2)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(58);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for ExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_expr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule term*/
			recog.base.set_state(59);
			recog.term()?;

			recog.base.set_state(64);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(60);
					_la = recog.base.input.la(1);
					if { !(_la==T__6 || _la==T__7) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule term*/
					recog.base.set_state(61);
					recog.term()?;

					}
					} 
				}
				recog.base.set_state(66);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- term ----------------
pub type TermContextAll<'input> = TermContext<'input>;


pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for TermContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_term(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for TermContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TermContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<TermContextExt<'input>>{

fn factor_all(&self) ->  Vec<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn factor(&self, i: usize) -> Option<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule factor*/
			recog.base.set_state(67);
			recog.factor()?;

			recog.base.set_state(72);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(68);
					_la = recog.base.input.la(1);
					if { !(_la==T__8 || _la==T__9) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule factor*/
					recog.base.set_state(69);
					recog.factor()?;

					}
					} 
				}
				recog.base.set_state(74);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- factor ----------------
pub type FactorContextAll<'input> = FactorContext<'input>;


pub type FactorContext<'input> = BaseParserRuleContext<'input,FactorContextExt<'input>>;

#[derive(Clone)]
pub struct FactorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for FactorContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for FactorContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_factor(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_factor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for FactorContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_factor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FactorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_factor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_factor }
}
antlr_rust::tid!{FactorContextExt<'a>}

impl<'input> FactorContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FactorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FactorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FactorContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<FactorContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn truth(&self) -> Option<Rc<TruthContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Number
/// Returns `None` if there is no child corresponding to token Number
fn Number(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Number, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> FactorContextAttrs<'input> for FactorContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn factor(&mut self,)
	-> Result<Rc<FactorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FactorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_factor);
        let mut _localctx: Rc<FactorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(83);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__10 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					recog.base.set_state(75);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(76);
					recog.expr()?;

					recog.base.set_state(77);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					}
					}
				}

			 T__0 | T__1 | T__2 | T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule truth*/
					recog.base.set_state(79);
					recog.truth_rec(0)?;

					}
				}

			 Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(80);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

			 Number 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(81);
					recog.base.match_token(Number,&mut recog.err_handler)?;

					}
				}

			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(82);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stmt ----------------
pub type StmtContextAll<'input> = StmtContext<'input>;


pub type StmtContext<'input> = BaseParserRuleContext<'input,StmtContextExt<'input>>;

#[derive(Clone)]
pub struct StmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for StmtContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for StmtContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stmt(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_stmt(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for StmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_stmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}
antlr_rust::tid!{StmtContextExt<'a>}

impl<'input> StmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StmtContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<StmtContextExt<'input>>{

fn select(&self) -> Option<Rc<SelectContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iterat(&self) -> Option<Rc<IteratContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set(&self) -> Option<Rc<SetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn write(&self) -> Option<Rc<WriteContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn read(&self) -> Option<Rc<ReadContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn asNumber(&self) -> Option<Rc<AsNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn asString(&self) -> Option<Rc<AsStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StmtContextAttrs<'input> for StmtContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmt(&mut self,)
	-> Result<Rc<StmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(92);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule select*/
					recog.base.set_state(85);
					recog.select()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule iterat*/
					recog.base.set_state(86);
					recog.iterat()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule set*/
					recog.base.set_state(87);
					recog.set()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule write*/
					recog.base.set_state(88);
					recog.write()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule read*/
					recog.base.set_state(89);
					recog.read()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule asNumber*/
					recog.base.set_state(90);
					recog.asNumber()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule asString*/
					recog.base.set_state(91);
					recog.asString()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- select ----------------
pub type SelectContextAll<'input> = SelectContext<'input>;


pub type SelectContext<'input> = BaseParserRuleContext<'input,SelectContextExt<'input>>;

#[derive(Clone)]
pub struct SelectContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for SelectContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for SelectContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_select(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_select(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for SelectContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_select(self);
	}
}

impl<'input> CustomRuleContext<'input> for SelectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_select }
	//fn type_rule_index() -> usize where Self: Sized { RULE_select }
}
antlr_rust::tid!{SelectContextExt<'a>}

impl<'input> SelectContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SelectContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SelectContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SelectContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<SelectContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope_all(&self) ->  Vec<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scope(&self, i: usize) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SelectContextAttrs<'input> for SelectContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn select(&mut self,)
	-> Result<Rc<SelectContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SelectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_select);
        let mut _localctx: Rc<SelectContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(94);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(95);
			recog.expr()?;

			recog.base.set_state(96);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			/*InvokeRule scope*/
			recog.base.set_state(97);
			recog.scope()?;

			recog.base.set_state(98);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			/*InvokeRule scope*/
			recog.base.set_state(99);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iterat ----------------
pub type IteratContextAll<'input> = IteratContext<'input>;


pub type IteratContext<'input> = BaseParserRuleContext<'input,IteratContextExt<'input>>;

#[derive(Clone)]
pub struct IteratContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for IteratContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for IteratContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iterat(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_iterat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for IteratContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_iterat(self);
	}
}

impl<'input> CustomRuleContext<'input> for IteratContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iterat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iterat }
}
antlr_rust::tid!{IteratContextExt<'a>}

impl<'input> IteratContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IteratContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IteratContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IteratContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<IteratContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IteratContextAttrs<'input> for IteratContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iterat(&mut self,)
	-> Result<Rc<IteratContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IteratContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_iterat);
        let mut _localctx: Rc<IteratContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(101);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(102);
			recog.expr()?;

			/*InvokeRule scope*/
			recog.base.set_state(103);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- set ----------------
pub type SetContextAll<'input> = SetContext<'input>;


pub type SetContext<'input> = BaseParserRuleContext<'input,SetContextExt<'input>>;

#[derive(Clone)]
pub struct SetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for SetContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for SetContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_set(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_set(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for SetContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_set(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set }
}
antlr_rust::tid!{SetContextExt<'a>}

impl<'input> SetContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<SetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SetContextAttrs<'input> for SetContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn set(&mut self,)
	-> Result<Rc<SetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_set);
        let mut _localctx: Rc<SetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(105);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			recog.base.set_state(106);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(107);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(108);
			recog.expr()?;

			recog.base.set_state(109);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- write ----------------
pub type WriteContextAll<'input> = WriteContext<'input>;


pub type WriteContext<'input> = BaseParserRuleContext<'input,WriteContextExt<'input>>;

#[derive(Clone)]
pub struct WriteContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for WriteContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for WriteContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_write(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_write(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for WriteContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_write(self);
	}
}

impl<'input> CustomRuleContext<'input> for WriteContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_write }
	//fn type_rule_index() -> usize where Self: Sized { RULE_write }
}
antlr_rust::tid!{WriteContextExt<'a>}

impl<'input> WriteContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WriteContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WriteContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WriteContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<WriteContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WriteContextAttrs<'input> for WriteContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn write(&mut self,)
	-> Result<Rc<WriteContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WriteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_write);
        let mut _localctx: Rc<WriteContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(111);
			recog.base.match_token(T__19,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(112);
			recog.expr()?;

			recog.base.set_state(113);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- read ----------------
pub type ReadContextAll<'input> = ReadContext<'input>;


pub type ReadContext<'input> = BaseParserRuleContext<'input,ReadContextExt<'input>>;

#[derive(Clone)]
pub struct ReadContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for ReadContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for ReadContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_read(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_read(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for ReadContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_read(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReadContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read }
}
antlr_rust::tid!{ReadContextExt<'a>}

impl<'input> ReadContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReadContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReadContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReadContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<ReadContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReadContextAttrs<'input> for ReadContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn read(&mut self,)
	-> Result<Rc<ReadContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_read);
        let mut _localctx: Rc<ReadContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(115);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(116);
			recog.expr()?;

			recog.base.set_state(117);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- decl ----------------
pub type DeclContextAll<'input> = DeclContext<'input>;


pub type DeclContext<'input> = BaseParserRuleContext<'input,DeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for DeclContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for DeclContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decl(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_decl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for DeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decl }
}
antlr_rust::tid!{DeclContextExt<'a>}

impl<'input> DeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<DeclContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclContextAttrs<'input> for DeclContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decl(&mut self,)
	-> Result<Rc<DeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_decl);
        let mut _localctx: Rc<DeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(119);
			recog.variable()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variable(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(121);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			recog.base.set_state(122);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(125);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__17 {
				{
				recog.base.set_state(123);
				recog.base.match_token(T__17,&mut recog.err_handler)?;

				/*InvokeRule expr*/
				recog.base.set_state(124);
				recog.expr()?;

				}
			}

			recog.base.set_state(127);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- asNumber ----------------
pub type AsNumberContextAll<'input> = AsNumberContext<'input>;


pub type AsNumberContext<'input> = BaseParserRuleContext<'input,AsNumberContextExt<'input>>;

#[derive(Clone)]
pub struct AsNumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for AsNumberContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for AsNumberContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asNumber(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_asNumber(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for AsNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_asNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for AsNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asNumber }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asNumber }
}
antlr_rust::tid!{AsNumberContextExt<'a>}

impl<'input> AsNumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AsNumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AsNumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AsNumberContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<AsNumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> AsNumberContextAttrs<'input> for AsNumberContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asNumber(&mut self,)
	-> Result<Rc<AsNumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AsNumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_asNumber);
        let mut _localctx: Rc<AsNumberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(129);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(130);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(131);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- asString ----------------
pub type AsStringContextAll<'input> = AsStringContext<'input>;


pub type AsStringContext<'input> = BaseParserRuleContext<'input,AsStringContextExt<'input>>;

#[derive(Clone)]
pub struct AsStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for AsStringContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for AsStringContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asString(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_asString(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for AsStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_asString(self);
	}
}

impl<'input> CustomRuleContext<'input> for AsStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asString }
}
antlr_rust::tid!{AsStringContextExt<'a>}

impl<'input> AsStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AsStringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AsStringContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AsStringContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<AsStringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> AsStringContextAttrs<'input> for AsStringContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asString(&mut self,)
	-> Result<Rc<AsStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AsStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_asString);
        let mut _localctx: Rc<AsStringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(133);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			recog.base.set_state(134);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			recog.base.set_state(135);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stmts ----------------
pub type StmtsContextAll<'input> = StmtsContext<'input>;


pub type StmtsContext<'input> = BaseParserRuleContext<'input,StmtsContextExt<'input>>;

#[derive(Clone)]
pub struct StmtsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for StmtsContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for StmtsContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stmts(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_stmts(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for StmtsContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_stmts(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmts }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmts }
}
antlr_rust::tid!{StmtsContextExt<'a>}

impl<'input> StmtsContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StmtsContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<StmtsContextExt<'input>>{

fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StmtsContextAttrs<'input> for StmtsContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmts(&mut self,)
	-> Result<Rc<StmtsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_stmts);
        let mut _localctx: Rc<StmtsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule stmt*/
			recog.base.set_state(137);
			recog.stmt()?;

			recog.base.set_state(141);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__12) | (1usize << T__15) | (1usize << T__16) | (1usize << T__19) | (1usize << T__20) | (1usize << Identifier))) != 0) {
				{
				{
				/*InvokeRule stmt*/
				recog.base.set_state(138);
				recog.stmt()?;

				}
				}
				recog.base.set_state(143);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- decls ----------------
pub type DeclsContextAll<'input> = DeclsContext<'input>;


pub type DeclsContext<'input> = BaseParserRuleContext<'input,DeclsContextExt<'input>>;

#[derive(Clone)]
pub struct DeclsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for DeclsContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for DeclsContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decls(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_decls(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for DeclsContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_decls(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decls }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decls }
}
antlr_rust::tid!{DeclsContextExt<'a>}

impl<'input> DeclsContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclsContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<DeclsContextExt<'input>>{

fn decl_all(&self) ->  Vec<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn decl(&self, i: usize) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclsContextAttrs<'input> for DeclsContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decls(&mut self,)
	-> Result<Rc<DeclsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_decls);
        let mut _localctx: Rc<DeclsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule decl*/
			recog.base.set_state(144);
			recog.decl()?;

			recog.base.set_state(148);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 {
				{
				{
				/*InvokeRule decl*/
				recog.base.set_state(145);
				recog.decl()?;

				}
				}
				recog.base.set_state(150);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scope ----------------
pub type ScopeContextAll<'input> = ScopeContext<'input>;


pub type ScopeContext<'input> = BaseParserRuleContext<'input,ScopeContextExt<'input>>;

#[derive(Clone)]
pub struct ScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for ScopeContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for ScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scope(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_scope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for ScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_scope(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scope }
}
antlr_rust::tid!{ScopeContextExt<'a>}

impl<'input> ScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScopeContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<ScopeContextExt<'input>>{

fn decls(&self) -> Option<Rc<DeclsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stmts(&self) -> Option<Rc<StmtsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ScopeContextAttrs<'input> for ScopeContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scope(&mut self,)
	-> Result<Rc<ScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_scope);
        let mut _localctx: Rc<ScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(151);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			recog.base.set_state(153);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__21 {
				{
				/*InvokeRule decls*/
				recog.base.set_state(152);
				recog.decls()?;

				}
			}

			recog.base.set_state(156);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__12) | (1usize << T__15) | (1usize << T__16) | (1usize << T__19) | (1usize << T__20) | (1usize << Identifier))) != 0) {
				{
				/*InvokeRule stmts*/
				recog.base.set_state(155);
				recog.stmts()?;

				}
			}

			recog.base.set_state(158);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- init ----------------
pub type InitContextAll<'input> = InitContext<'input>;


pub type InitContext<'input> = BaseParserRuleContext<'input,InitContextExt<'input>>;

#[derive(Clone)]
pub struct InitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for InitContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for InitContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_init(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_init(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for InitContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_init(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_init }
	//fn type_rule_index() -> usize where Self: Sized { RULE_init }
}
antlr_rust::tid!{InitContextExt<'a>}

impl<'input> InitContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<InitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,MiniImpParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> InitContextAttrs<'input> for InitContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn init(&mut self,)
	-> Result<Rc<InitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_init);
        let mut _localctx: Rc<InitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(160);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(161);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prog ----------------
pub type ProgContextAll<'input> = ProgContext<'input>;


pub type ProgContext<'input> = BaseParserRuleContext<'input,ProgContextExt<'input>>;

#[derive(Clone)]
pub struct ProgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MiniImpParserContext<'input> for ProgContext<'input>{}

impl<'input,'a> Listenable<dyn MiniImpListener<'input> + 'a> for ProgContext<'input>{
		fn enter(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prog(self);
		}
		fn exit(&self,listener: &mut (dyn MiniImpListener<'input> + 'a)) {
			listener.exit_prog(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MiniImpVisitor<'input> + 'a> for ProgContext<'input>{
	fn accept(&self,visitor: &mut (dyn MiniImpVisitor<'input> + 'a)) {
		visitor.visit_prog(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MiniImpParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prog }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prog }
}
antlr_rust::tid!{ProgContextExt<'a>}

impl<'input> ProgContextExt<'input>{
	fn new(parent: Option<Rc<dyn MiniImpParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgContextAttrs<'input>: MiniImpParserContext<'input> + BorrowMut<ProgContextExt<'input>>{

fn init(&self) -> Option<Rc<InitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ProgContextAttrs<'input> for ProgContext<'input>{}

impl<'input, I, H> MiniImpParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prog(&mut self,)
	-> Result<Rc<ProgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_prog);
        let mut _localctx: Rc<ProgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule init*/
			recog.base.set_state(163);
			recog.init()?;

			/*InvokeRule scope*/
			recog.base.set_state(164);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
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
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x21\u{a9}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x05\x02\x31\x0a\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x07\x02\x39\x0a\x02\x0c\x02\x0e\x02\x3c\x0b\x02\x03\x03\x03\
	\x03\x03\x03\x07\x03\x41\x0a\x03\x0c\x03\x0e\x03\x44\x0b\x03\x03\x04\x03\
	\x04\x03\x04\x07\x04\x49\x0a\x04\x0c\x04\x0e\x04\x4c\x0b\x04\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x56\x0a\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\x5f\x0a\
	\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\
	\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{80}\x0a\x0d\x03\x0d\x03\x0d\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\
	\x03\x10\x07\x10\u{8e}\x0a\x10\x0c\x10\x0e\x10\u{91}\x0b\x10\x03\x11\x03\
	\x11\x07\x11\u{95}\x0a\x11\x0c\x11\x0e\x11\u{98}\x0b\x11\x03\x12\x03\x12\
	\x05\x12\u{9c}\x0a\x12\x03\x12\x05\x12\u{9f}\x0a\x12\x03\x12\x03\x12\x03\
	\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x14\x02\x03\x02\x15\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\
	\x02\x04\x03\x02\x09\x0a\x03\x02\x0b\x0c\x02\u{ab}\x02\x30\x03\x02\x02\x02\
	\x04\x3d\x03\x02\x02\x02\x06\x45\x03\x02\x02\x02\x08\x55\x03\x02\x02\x02\
	\x0a\x5e\x03\x02\x02\x02\x0c\x60\x03\x02\x02\x02\x0e\x67\x03\x02\x02\x02\
	\x10\x6b\x03\x02\x02\x02\x12\x71\x03\x02\x02\x02\x14\x75\x03\x02\x02\x02\
	\x16\x79\x03\x02\x02\x02\x18\x7b\x03\x02\x02\x02\x1a\u{83}\x03\x02\x02\x02\
	\x1c\u{87}\x03\x02\x02\x02\x1e\u{8b}\x03\x02\x02\x02\x20\u{92}\x03\x02\x02\
	\x02\x22\u{99}\x03\x02\x02\x02\x24\u{a2}\x03\x02\x02\x02\x26\u{a5}\x03\x02\
	\x02\x02\x28\x29\x08\x02\x01\x02\x29\x31\x07\x03\x02\x02\x2a\x31\x07\x04\
	\x02\x02\x2b\x2c\x07\x05\x02\x02\x2c\x31\x05\x02\x02\x06\x2d\x2e\x07\x06\
	\x02\x02\x2e\x2f\x07\x1e\x02\x02\x2f\x31\x05\x04\x03\x02\x30\x28\x03\x02\
	\x02\x02\x30\x2a\x03\x02\x02\x02\x30\x2b\x03\x02\x02\x02\x30\x2d\x03\x02\
	\x02\x02\x31\x3a\x03\x02\x02\x02\x32\x33\x0c\x04\x02\x02\x33\x34\x07\x07\
	\x02\x02\x34\x39\x05\x02\x02\x05\x35\x36\x0c\x03\x02\x02\x36\x37\x07\x08\
	\x02\x02\x37\x39\x05\x02\x02\x04\x38\x32\x03\x02\x02\x02\x38\x35\x03\x02\
	\x02\x02\x39\x3c\x03\x02\x02\x02\x3a\x38\x03\x02\x02\x02\x3a\x3b\x03\x02\
	\x02\x02\x3b\x03\x03\x02\x02\x02\x3c\x3a\x03\x02\x02\x02\x3d\x42\x05\x06\
	\x04\x02\x3e\x3f\x09\x02\x02\x02\x3f\x41\x05\x06\x04\x02\x40\x3e\x03\x02\
	\x02\x02\x41\x44\x03\x02\x02\x02\x42\x40\x03\x02\x02\x02\x42\x43\x03\x02\
	\x02\x02\x43\x05\x03\x02\x02\x02\x44\x42\x03\x02\x02\x02\x45\x4a\x05\x08\
	\x05\x02\x46\x47\x09\x03\x02\x02\x47\x49\x05\x08\x05\x02\x48\x46\x03\x02\
	\x02\x02\x49\x4c\x03\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4a\x4b\x03\x02\
	\x02\x02\x4b\x07\x03\x02\x02\x02\x4c\x4a\x03\x02\x02\x02\x4d\x4e\x07\x0d\
	\x02\x02\x4e\x4f\x05\x04\x03\x02\x4f\x50\x07\x0e\x02\x02\x50\x56\x03\x02\
	\x02\x02\x51\x56\x05\x02\x02\x02\x52\x56\x07\x1e\x02\x02\x53\x56\x07\x1f\
	\x02\x02\x54\x56\x07\x21\x02\x02\x55\x4d\x03\x02\x02\x02\x55\x51\x03\x02\
	\x02\x02\x55\x52\x03\x02\x02\x02\x55\x53\x03\x02\x02\x02\x55\x54\x03\x02\
	\x02\x02\x56\x09\x03\x02\x02\x02\x57\x5f\x05\x0c\x07\x02\x58\x5f\x05\x0e\
	\x08\x02\x59\x5f\x05\x10\x09\x02\x5a\x5f\x05\x12\x0a\x02\x5b\x5f\x05\x14\
	\x0b\x02\x5c\x5f\x05\x1a\x0e\x02\x5d\x5f\x05\x1c\x0f\x02\x5e\x57\x03\x02\
	\x02\x02\x5e\x58\x03\x02\x02\x02\x5e\x59\x03\x02\x02\x02\x5e\x5a\x03\x02\
	\x02\x02\x5e\x5b\x03\x02\x02\x02\x5e\x5c\x03\x02\x02\x02\x5e\x5d\x03\x02\
	\x02\x02\x5f\x0b\x03\x02\x02\x02\x60\x61\x07\x0f\x02\x02\x61\x62\x05\x04\
	\x03\x02\x62\x63\x07\x10\x02\x02\x63\x64\x05\x22\x12\x02\x64\x65\x07\x11\
	\x02\x02\x65\x66\x05\x22\x12\x02\x66\x0d\x03\x02\x02\x02\x67\x68\x07\x12\
	\x02\x02\x68\x69\x05\x04\x03\x02\x69\x6a\x05\x22\x12\x02\x6a\x0f\x03\x02\
	\x02\x02\x6b\x6c\x07\x13\x02\x02\x6c\x6d\x07\x1e\x02\x02\x6d\x6e\x07\x14\
	\x02\x02\x6e\x6f\x05\x04\x03\x02\x6f\x70\x07\x15\x02\x02\x70\x11\x03\x02\
	\x02\x02\x71\x72\x07\x16\x02\x02\x72\x73\x05\x04\x03\x02\x73\x74\x07\x15\
	\x02\x02\x74\x13\x03\x02\x02\x02\x75\x76\x07\x17\x02\x02\x76\x77\x05\x04\
	\x03\x02\x77\x78\x07\x15\x02\x02\x78\x15\x03\x02\x02\x02\x79\x7a\x05\x18\
	\x0d\x02\x7a\x17\x03\x02\x02\x02\x7b\x7c\x07\x18\x02\x02\x7c\x7f\x07\x1e\
	\x02\x02\x7d\x7e\x07\x14\x02\x02\x7e\u{80}\x05\x04\x03\x02\x7f\x7d\x03\x02\
	\x02\x02\x7f\u{80}\x03\x02\x02\x02\u{80}\u{81}\x03\x02\x02\x02\u{81}\u{82}\
	\x07\x15\x02\x02\u{82}\x19\x03\x02\x02\x02\u{83}\u{84}\x07\x1e\x02\x02\u{84}\
	\u{85}\x07\x19\x02\x02\u{85}\u{86}\x07\x15\x02\x02\u{86}\x1b\x03\x02\x02\
	\x02\u{87}\u{88}\x07\x1e\x02\x02\u{88}\u{89}\x07\x1a\x02\x02\u{89}\u{8a}\
	\x07\x15\x02\x02\u{8a}\x1d\x03\x02\x02\x02\u{8b}\u{8f}\x05\x0a\x06\x02\u{8c}\
	\u{8e}\x05\x0a\x06\x02\u{8d}\u{8c}\x03\x02\x02\x02\u{8e}\u{91}\x03\x02\x02\
	\x02\u{8f}\u{8d}\x03\x02\x02\x02\u{8f}\u{90}\x03\x02\x02\x02\u{90}\x1f\x03\
	\x02\x02\x02\u{91}\u{8f}\x03\x02\x02\x02\u{92}\u{96}\x05\x16\x0c\x02\u{93}\
	\u{95}\x05\x16\x0c\x02\u{94}\u{93}\x03\x02\x02\x02\u{95}\u{98}\x03\x02\x02\
	\x02\u{96}\u{94}\x03\x02\x02\x02\u{96}\u{97}\x03\x02\x02\x02\u{97}\x21\x03\
	\x02\x02\x02\u{98}\u{96}\x03\x02\x02\x02\u{99}\u{9b}\x07\x1b\x02\x02\u{9a}\
	\u{9c}\x05\x20\x11\x02\u{9b}\u{9a}\x03\x02\x02\x02\u{9b}\u{9c}\x03\x02\x02\
	\x02\u{9c}\u{9e}\x03\x02\x02\x02\u{9d}\u{9f}\x05\x1e\x10\x02\u{9e}\u{9d}\
	\x03\x02\x02\x02\u{9e}\u{9f}\x03\x02\x02\x02\u{9f}\u{a0}\x03\x02\x02\x02\
	\u{a0}\u{a1}\x07\x1c\x02\x02\u{a1}\x23\x03\x02\x02\x02\u{a2}\u{a3}\x07\x1d\
	\x02\x02\u{a3}\u{a4}\x07\x1e\x02\x02\u{a4}\x25\x03\x02\x02\x02\u{a5}\u{a6}\
	\x05\x24\x13\x02\u{a6}\u{a7}\x05\x22\x12\x02\u{a7}\x27\x03\x02\x02\x02\x0e\
	\x30\x38\x3a\x42\x4a\x55\x5e\x7f\u{8f}\u{96}\u{9b}\u{9e}";

