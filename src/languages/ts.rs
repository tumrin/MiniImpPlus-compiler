use crate::{MiniImpPlus, TranslateMiniImpPlus};

pub struct TypeScript;

impl TranslateMiniImpPlus for TypeScript {
    fn translate(
        &self,
        value: MiniImpPlus,
        previous: Option<MiniImpPlus>,
        next: MiniImpPlus,
    ) -> String {
        match value {
            MiniImpPlus::True => "true".to_string(),
            MiniImpPlus::False => "false".to_string(),
            MiniImpPlus::Not => "!".to_string(),
            MiniImpPlus::Is => {
                if let MiniImpPlus::Identifier(_) = next {
                    "".to_string()
                } else if let MiniImpPlus::Not = previous.unwrap_or(MiniImpPlus::Unknown) {
                    "==".to_string()
                } else {
                    "===".to_string()
                }
            }
            MiniImpPlus::Or => "||".to_string(),
            MiniImpPlus::And => "&&".to_string(),
            MiniImpPlus::Plus => "+".to_string(),
            MiniImpPlus::Minus => "-".to_string(),
            MiniImpPlus::Multiply => "*".to_string(),
            MiniImpPlus::Divide => "/".to_string(),
            MiniImpPlus::OpenParenthesis => "(".to_string(),
            MiniImpPlus::CloseParenthesis => ")".to_string(),
            MiniImpPlus::If => "if".to_string(),
            MiniImpPlus::Then => "()".to_string(), // ?
            MiniImpPlus::Else => "else".to_string(),
            MiniImpPlus::While => "while".to_string(),
            MiniImpPlus::Set => "=".to_string(),
            MiniImpPlus::Equals => "=".to_string(),
            MiniImpPlus::Semicolon => ";".to_string(),
            MiniImpPlus::Write => "console.log".to_string(),
            MiniImpPlus::Read => todo!(),
            MiniImpPlus::Var => "let".to_string(),
            MiniImpPlus::AsNumber => "ParseInt".to_string(), // TODO:
            MiniImpPlus::AsString => "String".to_string(),
            MiniImpPlus::Begin => "{".to_string(),
            MiniImpPlus::End => "}".to_string(),
            MiniImpPlus::Program => todo!(),
            MiniImpPlus::Identifier(_) => todo!(),
            MiniImpPlus::Number(_) => todo!(),
            MiniImpPlus::WhiteSpace => "\n".to_string(),
            MiniImpPlus::String(_) => todo!(),
            MiniImpPlus::Unknown => "unknown".to_string(),
        }
    }
}
