#![allow(nonstandard_style)]
// Generated from MiniImp.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::miniimpparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link MiniImpParser}.
 */
pub trait MiniImpVisitor<'input>: ParseTreeVisitor<'input,MiniImpParserContextType>{
	/**
	 * Visit a parse tree produced by {@link MiniImpParser#truth}.
	 * @param ctx the parse tree
	 */
	fn visit_truth(&mut self, ctx: &TruthContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_expr(&mut self, ctx: &ExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_term(&mut self, ctx: &TermContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#factor}.
	 * @param ctx the parse tree
	 */
	fn visit_factor(&mut self, ctx: &FactorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_stmt(&mut self, ctx: &StmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#select}.
	 * @param ctx the parse tree
	 */
	fn visit_select(&mut self, ctx: &SelectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#iterat}.
	 * @param ctx the parse tree
	 */
	fn visit_iterat(&mut self, ctx: &IteratContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#set}.
	 * @param ctx the parse tree
	 */
	fn visit_set(&mut self, ctx: &SetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#write}.
	 * @param ctx the parse tree
	 */
	fn visit_write(&mut self, ctx: &WriteContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#read}.
	 * @param ctx the parse tree
	 */
	fn visit_read(&mut self, ctx: &ReadContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#decl}.
	 * @param ctx the parse tree
	 */
	fn visit_decl(&mut self, ctx: &DeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#asNumber}.
	 * @param ctx the parse tree
	 */
	fn visit_asNumber(&mut self, ctx: &AsNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#asString}.
	 * @param ctx the parse tree
	 */
	fn visit_asString(&mut self, ctx: &AsStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#stmts}.
	 * @param ctx the parse tree
	 */
	fn visit_stmts(&mut self, ctx: &StmtsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#decls}.
	 * @param ctx the parse tree
	 */
	fn visit_decls(&mut self, ctx: &DeclsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#scope}.
	 * @param ctx the parse tree
	 */
	fn visit_scope(&mut self, ctx: &ScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#init}.
	 * @param ctx the parse tree
	 */
	fn visit_init(&mut self, ctx: &InitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#prog}.
	 * @param ctx the parse tree
	 */
	fn visit_prog(&mut self, ctx: &ProgContext<'input>) { self.visit_children(ctx) }

}

pub trait MiniImpVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= MiniImpParserContextType>{
	/**
	 * Visit a parse tree produced by {@link MiniImpParser#truth}.
	 * @param ctx the parse tree
	 */
		fn visit_truth(&mut self, ctx: &TruthContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#term}.
	 * @param ctx the parse tree
	 */
		fn visit_term(&mut self, ctx: &TermContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#factor}.
	 * @param ctx the parse tree
	 */
		fn visit_factor(&mut self, ctx: &FactorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_stmt(&mut self, ctx: &StmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#select}.
	 * @param ctx the parse tree
	 */
		fn visit_select(&mut self, ctx: &SelectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#iterat}.
	 * @param ctx the parse tree
	 */
		fn visit_iterat(&mut self, ctx: &IteratContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#set}.
	 * @param ctx the parse tree
	 */
		fn visit_set(&mut self, ctx: &SetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#write}.
	 * @param ctx the parse tree
	 */
		fn visit_write(&mut self, ctx: &WriteContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#read}.
	 * @param ctx the parse tree
	 */
		fn visit_read(&mut self, ctx: &ReadContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#decl}.
	 * @param ctx the parse tree
	 */
		fn visit_decl(&mut self, ctx: &DeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#asNumber}.
	 * @param ctx the parse tree
	 */
		fn visit_asNumber(&mut self, ctx: &AsNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#asString}.
	 * @param ctx the parse tree
	 */
		fn visit_asString(&mut self, ctx: &AsStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#stmts}.
	 * @param ctx the parse tree
	 */
		fn visit_stmts(&mut self, ctx: &StmtsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#decls}.
	 * @param ctx the parse tree
	 */
		fn visit_decls(&mut self, ctx: &DeclsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#scope}.
	 * @param ctx the parse tree
	 */
		fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#init}.
	 * @param ctx the parse tree
	 */
		fn visit_init(&mut self, ctx: &InitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link MiniImpParser#prog}.
	 * @param ctx the parse tree
	 */
		fn visit_prog(&mut self, ctx: &ProgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> MiniImpVisitor<'input> for T
where
	T: MiniImpVisitorCompat<'input>
{
	fn visit_truth(&mut self, ctx: &TruthContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_truth(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expr(&mut self, ctx: &ExprContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_term(&mut self, ctx: &TermContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_factor(&mut self, ctx: &FactorContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_factor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stmt(&mut self, ctx: &StmtContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_select(&mut self, ctx: &SelectContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_select(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iterat(&mut self, ctx: &IteratContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_iterat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_set(&mut self, ctx: &SetContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_set(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_write(&mut self, ctx: &WriteContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_write(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_read(&mut self, ctx: &ReadContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_read(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decl(&mut self, ctx: &DeclContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_decl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asNumber(&mut self, ctx: &AsNumberContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_asNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asString(&mut self, ctx: &AsStringContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_asString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stmts(&mut self, ctx: &StmtsContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_stmts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decls(&mut self, ctx: &DeclsContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_decls(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scope(&mut self, ctx: &ScopeContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_scope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_init(&mut self, ctx: &InitContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_init(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prog(&mut self, ctx: &ProgContext<'input>){
		let result = <Self as MiniImpVisitorCompat>::visit_prog(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}