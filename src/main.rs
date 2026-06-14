mod cli;
mod date;
mod datetime;
mod lang;
mod sexagenary;

use std::io;

use clap::Parser;
use cli::{Cli, Commands, OutputMode};

use crate::{cli::WaybarOutput, datetime::LunisDateTime, sexagenary::TenGod};

const DEFAULT_TOOLTIP_FMT: &str =
    "{y-s} {y-b} ({y-y},{y-w}) | ({m-n}) {m-s} {m-b} | {d-s} {d-b} | {h-s} {h-b} | ke={ke}";

fn print_datetime_waybar(r: &LunisDateTime, fmt: &cli::CommonFormatArgs) {
    let text = r.format_string(&fmt.format, &fmt.lang);
    let tooltip_fmt = fmt
        .tooltip_format
        .clone()
        .unwrap_or_else(|| DEFAULT_TOOLTIP_FMT.to_string());
    let tooltip = r.format_string(&tooltip_fmt, &fmt.lang);
    let output = WaybarOutput {
        text: text.clone(),
        alt: text,
        tooltip,
        class: Some("lunis".to_string()),
    };
    println!("{}", serde_json::to_string(&output).unwrap());
}

fn print_datetime(r: &LunisDateTime, fmt: &cli::CommonFormatArgs) {
    match fmt.output {
        OutputMode::Default => println!("{}", r.format_string(&fmt.format, &fmt.lang)),
        OutputMode::Waybar => print_datetime_waybar(r, fmt),
    }
}

fn main() {
    match Cli::parse().command {
        Commands::Now { fmt } => {
            let r = LunisDateTime::now().unwrap();
            print_datetime(&r, &fmt);
        }
        Commands::Convert { fmt } => {
            let mut input_string = String::new();
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let r = LunisDateTime::from_rfc3339(&input_string.trim()).unwrap();
            print_datetime(&r, &fmt);
        }
        Commands::Relation(relation_args) => {
            let master = LunisDateTime::from_rfc3339(relation_args.master.trim()).unwrap();
            let target = LunisDateTime::from_rfc3339(relation_args.target.trim()).unwrap();

            let tengod = TenGod::resolve_tengod(master, target);
            let name = tengod.to_str(&relation_args.fmt.lang);
            let desc = tengod.describe(&relation_args.fmt.lang);

            match relation_args.fmt.output {
                OutputMode::Default => println!("{}: {}", name, desc),
                OutputMode::Waybar => {
                    let output = WaybarOutput {
                        text: name.to_string(),
                        alt: name.to_string(),
                        tooltip: format!("{}: {}", name, desc),
                        class: Some("lunis".to_string()),
                    };
                    println!("{}", serde_json::to_string(&output).unwrap());
                }
            }
        }
    }
}
