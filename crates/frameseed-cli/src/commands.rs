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
        #[arg(long, conflicts_with = "preset")]
        config: Option<PathBuf>,
        #[arg(long, conflicts_with = "config")]
        preset: Option<String>,
        #[arg(long, default_value = "mp4")]
        output_format: OutputFormat,
        #[arg(long)]
        contact_sheet: bool,
        #[arg(long, default_value = "12")]
        contact_sheet_step: u32,
        #[arg(long, default_value = "5")]
        contact_sheet_cols: u32,
    },
    Presets {
        #[command(subcommand)]
        command: PresetsCommands,
    },
    ListScenes,
}

#[derive(Subcommand)]
pub enum PresetsCommands {
    List,
    Save {
        #[arg(long)]
        name: String,
        #[arg(long)]
        config: PathBuf,
    },
}
