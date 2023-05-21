use clap::ValueEnum;

pub mod rust;
pub mod ts;

#[derive(ValueEnum, Debug, Clone)]
pub enum Languages {
    Rust,
    Typescript,
}
