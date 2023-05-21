use clap::ValueEnum;

pub mod rust;

#[derive(ValueEnum, Debug, Clone)]
pub enum Languages {
    Rust,
}
