use crate::mini_imp::{
    miniimpparser::{self, MiniImpParserContextType},
    miniimpvisitor::MiniImpVisitorCompat,
};
use antlr_rust::{
    rule_context::RuleContext,
    tree::{ParseTreeVisitorCompat, TerminalNode},
};

pub struct Javascript;
pub struct JSVisitor(pub String);

impl ParseTreeVisitorCompat<'_> for JSVisitor {
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
impl MiniImpVisitorCompat<'_> for JSVisitor {
    fn visit_truth(&mut self, ctx: &miniimpparser::TruthContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx).replace("is ", "");
        string
    }

    fn visit_expr(&mut self, ctx: &miniimpparser::ExprContext<'_>) -> Self::Return {
        match ctx.get_parent_ctx().unwrap().get_rule_index() {
            0 => "===".to_string() + "(" + &self.visit_children(ctx) + ")",
            5 | 6 => "(".to_string() + &self.visit_children(ctx) + ")",
            9 => {
                let value = self.visit_children(ctx);
                format!(
                    "const rl_{value} = readline.createInterface({{ input, output }});
                        {value} = await rl_{value}.question('');
                        rl_{value}.close()"
                )
            }
            _ => "(".to_string() + &self.visit_children(ctx) + ")",
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
        string.replace("write ", "console.log(").replace(';', ");")
    }

    fn visit_read(&mut self, ctx: &miniimpparser::ReadContext<'_>) -> Self::Return {
        self.visit_children(ctx).replace("read ", "")
    }

    fn visit_variable(&mut self, ctx: &miniimpparser::VariableContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string.replace("var ", "let ")
    }

    fn visit_asNumber(&mut self, ctx: &miniimpparser::AsNumberContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        format!(
            "let {} = Number(",
            string.split(' ').collect::<Vec<&str>>()[0]
        ) + string.replace("asNumber", ")").trim()
    }

    fn visit_asString(&mut self, ctx: &miniimpparser::AsStringContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        format!(
            "let {} = String(",
            string.split(' ').collect::<Vec<&str>>()[0]
        ) + string.replace("asString ", ")").trim()
    }

    fn visit_scope(&mut self, ctx: &miniimpparser::ScopeContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string.replace("begin ", "{").replace("end.", "}")
    }

    fn visit_prog(&mut self, ctx: &miniimpparser::ProgContext<'_>) -> Self::Return {
        let string = self.visit_children(ctx);
        string
            .replace(
                "program ", "import * as readline from 'node:readline/promises';\nimport { stdin as input, stdout as output } from 'node:process';\n",
            )
            .replace("DEMOAPP ", "")
            .replace("and ", "&&")
    }
}
