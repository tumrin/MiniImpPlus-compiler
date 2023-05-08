#![allow(nonstandard_style)]
// Generated from MiniImp.g4 by ANTLR 4.8
use super::miniimpparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait MiniImpListener<'input>: ParseTreeListener<'input, MiniImpParserContextType> {
    /**
     * Enter a parse tree produced by {@link MiniImpParser#truth}.
     * @param ctx the parse tree
     */
    fn enter_truth(&mut self, _ctx: &TruthContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#truth}.
     * @param ctx the parse tree
     */
    fn exit_truth(&mut self, _ctx: &TruthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term(&mut self, _ctx: &TermContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term(&mut self, _ctx: &TermContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#factor}.
     * @param ctx the parse tree
     */
    fn enter_factor(&mut self, _ctx: &FactorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#factor}.
     * @param ctx the parse tree
     */
    fn exit_factor(&mut self, _ctx: &FactorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_stmt(&mut self, _ctx: &StmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_stmt(&mut self, _ctx: &StmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#select}.
     * @param ctx the parse tree
     */
    fn enter_select(&mut self, _ctx: &SelectContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#select}.
     * @param ctx the parse tree
     */
    fn exit_select(&mut self, _ctx: &SelectContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#iterat}.
     * @param ctx the parse tree
     */
    fn enter_iterat(&mut self, _ctx: &IteratContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#iterat}.
     * @param ctx the parse tree
     */
    fn exit_iterat(&mut self, _ctx: &IteratContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#set}.
     * @param ctx the parse tree
     */
    fn enter_set(&mut self, _ctx: &SetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#set}.
     * @param ctx the parse tree
     */
    fn exit_set(&mut self, _ctx: &SetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#write}.
     * @param ctx the parse tree
     */
    fn enter_write(&mut self, _ctx: &WriteContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#write}.
     * @param ctx the parse tree
     */
    fn exit_write(&mut self, _ctx: &WriteContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#decl}.
     * @param ctx the parse tree
     */
    fn enter_decl(&mut self, _ctx: &DeclContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#decl}.
     * @param ctx the parse tree
     */
    fn exit_decl(&mut self, _ctx: &DeclContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#variable}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#variable}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#stmts}.
     * @param ctx the parse tree
     */
    fn enter_stmts(&mut self, _ctx: &StmtsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#stmts}.
     * @param ctx the parse tree
     */
    fn exit_stmts(&mut self, _ctx: &StmtsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#decls}.
     * @param ctx the parse tree
     */
    fn enter_decls(&mut self, _ctx: &DeclsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#decls}.
     * @param ctx the parse tree
     */
    fn exit_decls(&mut self, _ctx: &DeclsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#scope}.
     * @param ctx the parse tree
     */
    fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#scope}.
     * @param ctx the parse tree
     */
    fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#init}.
     * @param ctx the parse tree
     */
    fn enter_init(&mut self, _ctx: &InitContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#init}.
     * @param ctx the parse tree
     */
    fn exit_init(&mut self, _ctx: &InitContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link MiniImpParser#prog}.
     * @param ctx the parse tree
     */
    fn enter_prog(&mut self, _ctx: &ProgContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link MiniImpParser#prog}.
     * @param ctx the parse tree
     */
    fn exit_prog(&mut self, _ctx: &ProgContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : MiniImpListener<'input> }
