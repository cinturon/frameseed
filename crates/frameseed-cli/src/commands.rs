use clap::{Parser, Subcommand};
use std::path::PathBuf;


#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Mp4,
    Gif,
}

#[derive(Parser)]
#[command(name = "frameseed")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Render {
        #[arg(long)]
        config: PathBuf,
        #[arg(long, default_value = "mp4")]
        output_format: OutputFormat,
    },
}
