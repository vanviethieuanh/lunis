use crate::lang::LunarLang;

use clap::{Args, Parser, Subcommand};

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
}
