mod cli;
mod date;
mod datetime;
mod lang;
mod sexagenary;
mod time;

use std::io;

use clap::Parser;
use cli::{Cli, Commands};

use crate::datetime::LunisDateTime;

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
    }
}
