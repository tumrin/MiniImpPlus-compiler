use std::format;

use crate::{MiniImpPlus, TranslateMiniImpPlus};

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
            MiniImpPlus::If => "if".to_string(),
            MiniImpPlus::Then => "".to_string(),
            MiniImpPlus::Else => "else".to_string(),
            MiniImpPlus::While => "while".to_string(),
            MiniImpPlus::Set => "".to_string(),
            MiniImpPlus::Equals => "=".to_string(),
            MiniImpPlus::Semicolon => ";\n".to_string(),
            MiniImpPlus::Write => "println!".to_string(),
            MiniImpPlus::Read => "".to_string(), // Handled in identifier
            MiniImpPlus::Var => "let mut ".to_string(),
            MiniImpPlus::AsNumber => todo!(),
            MiniImpPlus::AsString => todo!(),
            MiniImpPlus::Begin => "{\n".to_string(),
            MiniImpPlus::End => "}\n".to_string(),
            MiniImpPlus::Program => "fn main()".to_string(),
            MiniImpPlus::Identifier(value) => previous.map_or_else(
                || format!("{}", value),
                |prev| match prev {
                    MiniImpPlus::Write => {
                        format!("(\"{{{}}}\")", value)
                    },
                    MiniImpPlus::Read => {format!("let mut {value} = String::new();\n io::stdin().read_line(&mut line).unwrap();\n")},
                    MiniImpPlus::Is => {
                        format!("({} == ", value)
                    }
                    MiniImpPlus::Program => "".to_string(),
                    MiniImpPlus::Identifier(_) => format!("{})", value),
                    _ => match next {
                        MiniImpPlus::AsNumber => format!("{value}.parse::<i32>().unwrap()"),
                        MiniImpPlus::AsString => format!("{value}.to_string()"),
                        _ => format!("{}", value),
                    }
                },
            ),
            MiniImpPlus::Number(number) => number,
            MiniImpPlus::Unknown => "".to_string(),
        }
    }
}
// 1 => {
//     print!("true");
// }
// 2 => {
//     print!("false");
// }
// 3 => {
//     print!("!");
// }
// 4 => {
// }
// 5 => {
//     print!("+");
// }
// 6 => {
//     print!("-");
// }
// 7 => {
//     print!("*");
// }
// 8 => {
//     print!("/");
// }
// 9 => {
//     print!("(");
// }
// 10 => {
//     print!(")");
// }
// 11 => {
//     print!("if");
// }
// 12 => {
//     print!("then");
// }
// 13 => {
//     print!("else");
// }
// 14 => {
//     print!("while ");
// }
// 15 => {
//     // Skip set
// }
// 16 => {
//     print!(" = ");
// }
// 17 => {
//     print!(";\n");
// }
// 18 => {
//     print!("println!");
// }
// 19 => {
//     PRINT!("LET MUT ");
// }
// 20 => {
//     PRINT!(" {{\N");
// }
// 21 => {
//     PRINT!("}}\N");
// }
// 22 => {
//     PRINT!("FN MAIN()");
// }
// 23 =>
// PREVIOUS.CLONE().MAP_OR_ELSE(
//     || PRINT!("{}", CURRENT.TEXT),
//     |PREV| MATCH PREV.TOKEN_TYPE {
//         18 => {
//             PRINT!("(\"{{{}}}\")", CURRENT.TEXT);
//         }
//         4 => {
//             PRINT!("({} == ", CURRENT.TEXT)
//         }
//         22 => (),
//         23 => PRINT!("{})", CURRENT.TEXT),
//         _ => PRINT!("{}", CURRENT.TEXT),
//     },
// ),
// 24 => {
//     PRINT!("{}", CURRENT.TEXT);
// }
// 25 => {
//     PRINT!("WS");
// }
// _ => {
//     //PRINT!("UNKNOWN");
// }
