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
        /// Override the seed from the config file. Pass a number or "random".
        #[arg(long)]
        seed: Option<String>,
        #[arg(long, default_value = "mp4")]
        output_format: OutputFormat,
        /// Output file path. Defaults to output/video.mp4 or output/animation.gif.
        #[arg(long)]
        output: Option<PathBuf>,
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
    ListPresets,
    ListPalettes,
    Preview {
        #[arg(long, conflicts_with_all = ["preset", "config"])]
        scene: Option<String>,
        #[arg(long, conflicts_with_all = ["scene", "config"])]
        preset: Option<String>,
        #[arg(long, conflicts_with_all = ["scene", "preset"])]
        config: Option<PathBuf>,
        /// Seed value. Pass a number or "random" to pick one at runtime.
        #[arg(long, default_value = "42")]
        seed: String,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 640)]
        width: u32,
        #[arg(long, default_value_t = 360)]
        height: u32,
        #[arg(long, default_value = "0")]
        frame_index: u32,
    },
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
