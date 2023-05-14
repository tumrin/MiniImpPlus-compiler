use crate::{MiniImpPlus, TranslateMiniImpPlus};

pub struct Rust;

impl TranslateMiniImpPlus for Rust {
    fn translate(&self, value: MiniImpPlus) -> String {
        match value {
            MiniImpPlus::True => "true".to_string(),
            MiniImpPlus::False => "false".to_string(),
            MiniImpPlus::Not => "!".to_string(),
            MiniImpPlus::Is => todo!(),
            MiniImpPlus::Or => todo!(),
            MiniImpPlus::And => todo!(),
            MiniImpPlus::Plus => todo!(),
            MiniImpPlus::Minus => todo!(),
            MiniImpPlus::Multiply => todo!(),
            MiniImpPlus::Divide => todo!(),
            MiniImpPlus::OpenParenthesis => todo!(),
            MiniImpPlus::CloseParenthesis => todo!(),
            MiniImpPlus::If => todo!(),
            MiniImpPlus::Then => todo!(),
            MiniImpPlus::Else => todo!(),
            MiniImpPlus::While => todo!(),
            MiniImpPlus::Set => todo!(),
            MiniImpPlus::Equals => todo!(),
            MiniImpPlus::Semicolon => todo!(),
            MiniImpPlus::Write => todo!(),
            MiniImpPlus::Read => todo!(),
            MiniImpPlus::Var => todo!(),
            MiniImpPlus::AsNumber => todo!(),
            MiniImpPlus::AsString => todo!(),
            MiniImpPlus::Begin => todo!(),
            MiniImpPlus::End => todo!(),
            MiniImpPlus::Program => todo!(),
            MiniImpPlus::Unknown => todo!(),
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
//     if next == 23 {
//         // Skip if next would be variable
//     } else if previous.map_or(false, |prev| prev.token_type == 3) {
//         // Only use one equal sign for negation
//         print!("=");
//     } else {
//         print!("==");
//     }
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
// 23 => PREVIOUS.CLONE().MAP_OR_ELSE(
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
