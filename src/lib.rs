use antlr_rust::token::GenericToken;
use std::{fmt::Display, usize};

pub mod languages;

pub enum MiniImpPlus {
    True,
    False,
    Not,
    Is,
    Or,
    And,
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    If,
    Then,
    Else,
    While,
    Set,
    Equals,
    Semicolon,
    Write,
    Read,
    Var,
    AsNumber,
    AsString,
    Begin,
    End,
    Program,
    Identifier,
    Number,
    Unknown,
}
impl Display for MiniImpPlus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<isize> for MiniImpPlus {
    fn from(value: isize) -> Self {
        match value {
            1 => MiniImpPlus::True,
            2 => MiniImpPlus::False,
            3 => MiniImpPlus::Not,
            4 => MiniImpPlus::Is,
            5 => MiniImpPlus::Or,
            6 => MiniImpPlus::And,
            7 => MiniImpPlus::Plus,
            8 => MiniImpPlus::Minus,
            9 => MiniImpPlus::Multiply,
            10 => MiniImpPlus::Divide,
            11 => MiniImpPlus::OpenParenthesis,
            12 => MiniImpPlus::CloseParenthesis,
            13 => MiniImpPlus::If,
            14 => MiniImpPlus::Then,
            15 => MiniImpPlus::Else,
            16 => MiniImpPlus::While,
            17 => MiniImpPlus::Set,
            18 => MiniImpPlus::Equals,
            19 => MiniImpPlus::Semicolon,
            20 => MiniImpPlus::Write,
            21 => MiniImpPlus::Read,
            22 => MiniImpPlus::Var,
            23 => MiniImpPlus::AsNumber,
            24 => MiniImpPlus::AsString,
            25 => MiniImpPlus::Begin,
            26 => MiniImpPlus::End,
            27 => MiniImpPlus::Program,
            28 => MiniImpPlus::Identifier,
            29 => MiniImpPlus::Number,
            _ => MiniImpPlus::Unknown,
        }
    }
}
pub trait TranslateMiniImpPlus {
    fn translate(
        &self,
        mini_imp: MiniImpPlus,
        previous: Option<MiniImpPlus>,
        next: MiniImpPlus,
    ) -> String;
}
