//! This module is for implementing mini imp plus to Rust translation
use crate::{MiniImpPlus, TranslateMiniImpPlus};
use std::format;

#[derive(Debug, Clone)]
pub struct Rust;

impl TranslateMiniImpPlus for Rust {
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
                    // Skip if next is an identifier
                    "".to_string()
                } else if let MiniImpPlus::Not = previous.unwrap_or(MiniImpPlus::Unknown) {
                    // Only use one equal sign for negation
                    "=".to_string()
                } else {
                    "==".to_string()
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
            MiniImpPlus::If => "if ".to_string(),
            MiniImpPlus::Then => "".to_string(),
            MiniImpPlus::Else => "else".to_string(),
            MiniImpPlus::While => "while ".to_string(),
            MiniImpPlus::Set => "".to_string(),
            MiniImpPlus::Equals => "=".to_string(),
            MiniImpPlus::Semicolon => ";\n".to_string(),
            MiniImpPlus::Write => "println!".to_string(),
            MiniImpPlus::Read => "".to_string(), // Handled in identifier
            MiniImpPlus::Var => "let mut ".to_string(),
            MiniImpPlus::AsNumber => "".to_string(), // Hanled in identifier
            MiniImpPlus::AsString => "".to_string(), // Handled in identifier
            MiniImpPlus::Begin => "{\n".to_string(),
            MiniImpPlus::End => "}\n".to_string(),
            MiniImpPlus::Program => "fn main()".to_string(),
            MiniImpPlus::Identifier(value) => match previous {
                Some(MiniImpPlus::Write) => {
                    format!("(\"{{{}}}\")", value)
                }
                Some(MiniImpPlus::Read) => {
                    format!("let mut {value} = String::new();\nstd::io::stdin().read_line(&mut {value}).unwrap();\n {value} = {value}.trim().to_string()")
                }
                Some(MiniImpPlus::Is) => {
                    format!("{} == ", value)
                }
                Some(MiniImpPlus::Program) => "".to_string(),
                Some(MiniImpPlus::Identifier(_)) => value,
                Some(_) | None => match next {
                    MiniImpPlus::AsNumber => format!("{value}.parse::<i32>().unwrap()"),
                    MiniImpPlus::AsString => format!("{value}.to_string()"),
                    _ => value,
                },
            },
            MiniImpPlus::Number(number) => number,
            MiniImpPlus::WhiteSpace => "".to_string(),
            MiniImpPlus::String(value) => match previous {
                Some(MiniImpPlus::Write) => format!("({value})"),
                None | Some(_) => value,
            },
            MiniImpPlus::Unknown => "".to_string(),
        }
    }
}
