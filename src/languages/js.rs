use crate::{MiniImpPlus, TranslateMiniImpPlus};

pub struct Javascript;

impl TranslateMiniImpPlus for Javascript {
    fn translate(
        &self,
        value: MiniImpPlus,
        previous: Option<MiniImpPlus>,
        next: MiniImpPlus,
    ) -> String {
        match value {
            MiniImpPlus::True => match previous {
                Some(MiniImpPlus::Identifier(_)) => "true)".to_string(),
                None | Some(_) => "true".to_string(),
            },
            MiniImpPlus::False => match previous {
                Some(MiniImpPlus::Identifier(_)) => "false)".to_string(),
                None | Some(_) => "false".to_string(),
            },
            MiniImpPlus::Not => "!".to_string(),
            MiniImpPlus::Is => {
                if let MiniImpPlus::Identifier(_) = next {
                    // Skip if there is already an identifier coming next
                    "".to_string()
                } else if let MiniImpPlus::Not = previous.unwrap_or(MiniImpPlus::Unknown) {
                    // In case there is a negation (not) operator, omit the first character
                    "==".to_string()
                } else {
                    // Default case
                    "===".to_string()
                }
            }
            MiniImpPlus::Or => " || ".to_string(),
            MiniImpPlus::And => " && ".to_string(),
            MiniImpPlus::Plus => "+".to_string(),
            MiniImpPlus::Minus => "-".to_string(),
            MiniImpPlus::Multiply => "*".to_string(),
            MiniImpPlus::Divide => "/".to_string(),
            MiniImpPlus::OpenParenthesis => "(".to_string(),
            MiniImpPlus::CloseParenthesis => ")".to_string(),
            MiniImpPlus::If => "if (".to_string(),
            MiniImpPlus::Then => " )".to_string(),
            MiniImpPlus::Else => "else".to_string(),
            MiniImpPlus::While => "while".to_string(),
            MiniImpPlus::Set => "".to_string(),
            MiniImpPlus::Equals => " = ".to_string(),
            MiniImpPlus::Semicolon => ";\n".to_string(),
            MiniImpPlus::Write => "console.log".to_string(),
            MiniImpPlus::Read => "".to_string(), // in identifier
            MiniImpPlus::Var => "let ".to_string(),
            MiniImpPlus::AsNumber => "".to_string(), // in identifier
            MiniImpPlus::AsString => "".to_string(), // in identifier
            MiniImpPlus::Begin => "{ \n".to_string(),
            MiniImpPlus::End => "\n}".to_string(),
            MiniImpPlus::Program => "
import * as readline from 'node:readline/promises';\n
import { stdin as input, stdout as output } from 'node:process';\n
"
            .to_string(),
            MiniImpPlus::Identifier(value) => match previous {
                Some(MiniImpPlus::Write) => {
                    format!("({})", value)
                }
                Some(MiniImpPlus::Read) => {
                    format!(
                        "const rl_{value} = readline.createInterface({{ input, output }});
                        {value} = await rl_{value}.question('');
                        rl_{value}.close()"
                    )
                }
                Some(MiniImpPlus::Is) => {
                    format!("({} === ", value)
                }
                Some(MiniImpPlus::Program) => "".to_string(),
                Some(MiniImpPlus::Identifier(_)) => format!("{value})"),
                Some(_) | None => match next {
                    MiniImpPlus::AsNumber => format!("Number({value})"),
                    MiniImpPlus::AsString => format!("{value}.toString()"),
                    _ => value,
                },
            },
            MiniImpPlus::Number(_) => "".to_string(), //TODO:
            MiniImpPlus::WhiteSpace => "\n".to_string(),
            MiniImpPlus::String(value) => match previous {
                Some(MiniImpPlus::Write) => format!("({value})"),
                Some(MiniImpPlus::Identifier(_)) => format!("{value})"),
                None | Some(_) => value,
            },
            MiniImpPlus::Unknown => "".to_string(),
        }
    }
}
