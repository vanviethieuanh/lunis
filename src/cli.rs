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

    /// Output mode
    #[arg(short, long, value_enum, default_value_t = OutputMode::Default)]
    pub output: OutputMode,

    /// Format for tooltip (Waybar mode); defaults to a rich format if not set
    #[arg(long)]
    pub tooltip_format: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputMode {
    Default,
    Waybar,
}

#[derive(Serialize)]
pub struct WaybarOutput {
    pub text: String,
    pub alt: String,
    pub tooltip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Pillar {
    Year,
    Month,
    Day,
    Hour,
}

#[derive(Args, Debug)]
pub struct RelationArgs {
    #[command(flatten)]
    pub fmt: CommonFormatArgs,

    /// First date in RFC3339
    pub master: String,

    /// Second date in RFC3339
    pub target: String,

    /// Target pillar (year, month, day, hour)
    #[arg(short, long, value_enum, default_value_t = Pillar::Day)]
    pub pillar: Pillar,

    /// Show all four pillars with hidden stems
    #[arg(short, long)]
    pub all: bool,

    /// Show Na Yin attribute for the target date
    #[arg(long)]
    pub nayin: bool,
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
