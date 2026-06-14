use crate::lang::LunarLang;

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about = "East Asian Lunisolar Calendar CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct CommonFormatArgs {
    #[arg(short, long)]
    pub lang: LunarLang,

    /// Output format placeholders
    #[arg(
        short,
        long,
        default_value = "{y-s} {y-b} ({y-y},{y-w}) | ({m-n}) {m-s} {m-b} | {d-s} {d-b} | {h-s} {h-b} | ke={ke}"
    )]
    pub format: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputMode {
    Default,
    Waybar,
}

#[derive(Serialize)]
struct WaybarOutput {
    text: String,
    alt: String,
    tooltip: String,
}

#[derive(Args, Debug)]
pub struct RelationArgs {
    #[arg(short, long)]
    pub lang: LunarLang,

    /// First date in RFC3339
    pub master: String,

    /// Second date in RFC3339
    pub target: String,

    /// Output mode: default waybar
    #[arg(short, long, value_enum, default_value_t = OutputMode::Default)]
    pub output: OutputMode,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get today's date
    Now {
        #[command(flatten)]
        fmt: CommonFormatArgs,
    },

    /// Convert a given RFC3339, try: date -Iseconds
    Convert {
        #[command(flatten)]
        fmt: CommonFormatArgs,
    },

    // Check the interraction between 2 days
    Relation(RelationArgs),
}
