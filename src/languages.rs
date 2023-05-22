use clap::ValueEnum;

pub mod js;
pub mod rust;

#[derive(ValueEnum, Debug, Clone)]
pub enum Languages {
    Rust,
    Javascript,
}
