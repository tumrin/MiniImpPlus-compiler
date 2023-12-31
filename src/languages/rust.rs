//! This module is for implementing mini imp plus to Rust translation
use crate::mini_imp::{
    miniimpparser::{self, MiniImpParserContextType},
    miniimpvisitor::MiniImpVisitorCompat,
};
use antlr_rust::{
    parser_rule_context::ParserRuleContext,
    rule_context::{CustomRuleContext, RuleContext},
    tree::{ParseTreeVisitorCompat, TerminalNode},
};
use std::format;

pub struct Rust;
pub struct RustVisitor(pub String);

impl ParseTreeVisitorCompat<'_> for RustVisitor {
    type Node = MiniImpParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        _node.symbol.text.to_string() + " "
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }
}
impl MiniImpVisitorCompat<'_> for RustVisitor {
    fn visit_truth(&mut self, ctx: &miniimpparser::TruthContext<'_>) -> Self::Return {
        let string = self
            .visit_children(ctx)
            .replace("is ", "")
            .replace("and ", "&&");
        string
    }

    fn visit_expr(&mut self, ctx: &miniimpparser::ExprContext<'_>) -> Self::Return {
        match ctx.get_parent_ctx().unwrap().get_rule_index() {
            0 => "== ".to_string() + &self.visit_children(ctx),
            9 => {
                let value = self.visit_children(ctx);
                format!("let mut {value} = String::new();\nstd::io::stdin().read_line(&mut {value}).unwrap();\n {value} = {value}.trim().to_string()")
            }
            _ => self.visit_children(ctx),
        }
    }

    fn visit_select(&mut self, ctx: &miniimpparser::SelectContext<'_>) -> Self::Return {
        self.visit_children(ctx).replace("then ", "")
    }

    fn visit_set(&mut self, ctx: &miniimpparser::SetContext<'_>) -> Self::Return {
        self.visit_children(ctx).replace("set ", "")
    }

    fn visit_write(&mut self, ctx: &miniimpparser::WriteContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string
            .replace("write ", "println!(\"{}\", ")
            .replace(';', ");")
    }

    fn visit_read(&mut self, ctx: &miniimpparser::ReadContext<'_>) -> Self::Return {
        self.visit_children(ctx).replace("read ", "")
    }

    fn visit_variable(&mut self, ctx: &miniimpparser::VariableContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string.replace("var ", "let mut ")
    }

    fn visit_asNumber(&mut self, ctx: &miniimpparser::AsNumberContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        format!("let mut {} = ", string.split(' ').collect::<Vec<&str>>()[0])
            + string
                .replace("asNumber ", ".parse::<i32>().unwrap()")
                .trim()
    }

    fn visit_asString(&mut self, ctx: &miniimpparser::AsStringContext<'_>) -> Self::Return {
        self.visit_children(ctx)
            .replace("asString ", ".to_string()")
    }

    fn visit_scope(&mut self, ctx: &miniimpparser::ScopeContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string.replace("begin ", "{ ").replace("end. ", "}")
    }

    fn visit_prog(&mut self, ctx: &miniimpparser::ProgContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string
            .replace("program ", "fn ")
            .replace("DEMOAPP ", "main()")
    }
}

// example test for fun
#[cfg(test)]
mod tests {

    use crate::mini_imp::{miniimplexer::MiniImpLexer, miniimpparser::MiniImpParser};

    use super::*;
    use antlr_rust::{
        common_token_stream::CommonTokenStream, tree::ParseTreeVisitorCompat, InputStream,
    };
    #[test]
    fn test_if() {
        let test = "if is X true then begin write X; end. else begin write X; end.";
        let stream = InputStream::new(test);
        let lexer = MiniImpLexer::new(stream);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = MiniImpParser::new(token_stream);

        let root = parser.select().unwrap();
        let mut visitor = RustVisitor(String::new());
        let output = visitor.visit(&*root);
        println!("{output}");
        assert_eq!(
            output,
            "if X == true { println!(\"{}\", X ); }else { println!(\"{}\", X ); }"
        );
    }
}
