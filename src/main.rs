mod cli;
mod date;
mod datetime;
mod lang;
mod sexagenary;

use std::io;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{datetime::LunisDateTime, sexagenary::TenGod};

fn main() {
    match Cli::parse().command {
        Commands::Now { fmt } => {
            let r = LunisDateTime::now().unwrap();
            println!("{}", r.format_string(&fmt.format, &fmt.lang));
        }
        Commands::Convert { fmt } => {
            let mut input_string = String::new();
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let r = LunisDateTime::from_rfc3339(&input_string.trim()).unwrap();

            println!("{}", r.format_string(&fmt.format, &fmt.lang));
        }
        Commands::Relation(relation_args) => {
            let master = LunisDateTime::from_rfc3339(relation_args.master.trim()).unwrap();
            let target = LunisDateTime::from_rfc3339(relation_args.target.trim()).unwrap();

            let tengod = TenGod::resolve_tengod(master, target);
            println!(
                "{}: {}",
                tengod.to_str(&relation_args.lang),
                tengod.describe(&relation_args.lang)
            )
        }
    }
}
