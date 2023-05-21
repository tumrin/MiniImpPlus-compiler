use crate::{MiniImpPlus, TranslateMiniImpPlus};

pub struct Typescript;

impl TranslateMiniImpPlus for Typescript {
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
            MiniImpPlus::While => "".to_string(),
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
const readline = require('node:readline');\n
const { stdin: input, stdout: output } = require('node:process');\n
const rl = readline.createInterface({ input, output });"
                .to_string(),
            MiniImpPlus::Identifier(value) => match previous {
                Some(MiniImpPlus::Write) => {
                    format!("({})", value)
                }
                Some(MiniImpPlus::While) => {
                    format!("while( {} )", value)
                }
                Some(MiniImpPlus::Read) => {
                    format!(
                        "rl.on(\"line\", (answer: string) => {{
                        {value} = answer;
                        console.log(\"hello bois\");
                      }});"
                    )
                }
                Some(MiniImpPlus::Is) => {
                    format!("{} === ", value)
                }
                Some(MiniImpPlus::Program) => "".to_string(),
                Some(MiniImpPlus::Identifier(_)) => format!("{}", value),
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
                None | Some(_) => value,
            },
            MiniImpPlus::Unknown => "".to_string(),
        }
    }
}
