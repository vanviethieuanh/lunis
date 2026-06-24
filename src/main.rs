mod auspicious;
mod cli;
mod date;
mod datetime;
mod lang;
mod sexagenary;

use std::io;

use clap::Parser;
use cli::{Cli, Commands, OutputMode, Pillar};

use crate::{
    cli::WaybarOutput,
    datetime::LunisDateTime,
    lang::LunarLang,
    sexagenary::{Branch, NaYin, PillarGods, Stem, TenGod},
};

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

fn get_pillar_data<'a>(dt: &'a LunisDateTime, pillar: &Pillar) -> (u32, Stem, Branch) {
    match pillar {
        Pillar::Year => dt.get_year(),
        Pillar::Month => dt.get_month(),
        Pillar::Day => dt.get_day(),
        Pillar::Hour => dt.get_hour(),
    }
}

fn format_stem_gods(
    stem: Stem,
    god: TenGod,
    hidden: &[(Stem, TenGod)],
    lang: &LunarLang,
) -> String {
    let stem_name = stem.to_str(lang);
    let god_name = god.to_str(lang);
    if hidden.is_empty() {
        format!("{}: {}", stem_name, god_name)
    } else {
        let hidden_parts: Vec<String> = hidden
            .iter()
            .map(|(hs, hg)| format!("{}({})", hs.to_str(lang), hg.to_str(lang)))
            .collect();
        format!("{}: {} [{}]", stem_name, god_name, hidden_parts.join(", "))
    }
}

fn print_pillar_gods(gods: &PillarGods, lang: &LunarLang) {
    let pillars = [
        ("Year", &gods.year),
        ("Month", &gods.month),
        ("Day", &gods.day),
        ("Hour", &gods.hour),
    ];
    for (label, p) in &pillars {
        let line = format_stem_gods(p.stem, p.stem_god, &p.hidden_gods, lang);
        println!("  {:5} ({})", label, line);
    }
}

fn print_nayin(gods: &PillarGods, lang: &LunarLang) {
    let nayins = [
        ("Year", gods.year.stem, gods.year.branch),
        ("Month", gods.month.stem, gods.month.branch),
        ("Day", gods.day.stem, gods.day.branch),
        ("Hour", gods.hour.stem, gods.hour.branch),
    ];
    let parts: Vec<String> = nayins
        .iter()
        .map(|(_, s, b)| {
            let n = NaYin::from_sexagenary(*s, *b);
            format!("{} ({})", n.to_str(lang), n.element.to_str(lang))
        })
        .collect();
    println!("  Na Yin: {}", parts.join(" | "));
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
        Commands::Judge { fmt, born, target } => {
            let master = LunisDateTime::from_rfc3339(&born.trim()).unwrap();
            let target_dt = match target {
                Some(t) => LunisDateTime::from_rfc3339(&t.trim()).unwrap(),
                None => LunisDateTime::now().unwrap(),
            };
            let rating = auspicious::evaluate(master, target_dt, &fmt.lang);
            match fmt.output {
                OutputMode::Default => {
                    println!("Rating: {} ({}/100)", rating.label, rating.score);
                    println!("{}", rating.detail);
                }
                OutputMode::Waybar => {
                    let text = target_dt.format_string(&fmt.format, &fmt.lang);
                    let alt = format!("{} ({}/100)", rating.label, rating.score);
                    let tooltip_fmt = fmt
                        .tooltip_format
                        .clone()
                        .unwrap_or_else(|| DEFAULT_TOOLTIP_FMT.to_string());
                    let tooltip_date = target_dt.format_string(&tooltip_fmt, &fmt.lang);

                    let best = auspicious::best_hours(master, target_dt, &fmt.lang);
                    let hours_text = best
                        .iter()
                        .take(3)
                        .map(|(b, r)| {
                            let idx = *b as u32;
                            let start = (idx * 2 + 23) % 24;
                            let end = (start + 2) % 24;
                            format!(
                                "  {} {:02}:00-{:02}:00 - {} ({}/100)",
                                b.to_str(&fmt.lang),
                                start,
                                end,
                                r.label,
                                r.score
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let hours_header = match fmt.lang {
                        LunarLang::Vi => "Giờ tốt nhất:",
                        LunarLang::Zh => "最佳时辰:",
                        LunarLang::Ko => "좋은 시진:",
                        LunarLang::Jp => "最良の時辰:",
                    };

                    let tooltip = format!(
                        "{}\n{}\n{}\n{}\n{}",
                        tooltip_date, alt, rating.detail, hours_header, hours_text
                    );
                    let output = WaybarOutput {
                        text,
                        alt,
                        tooltip,
                        class: Some("lunis-judge".to_string()),
                    };
                    println!("{}", serde_json::to_string(&output).unwrap());
                }
            }
        }
        Commands::Relation(relation_args) => {
            let master = LunisDateTime::from_rfc3339(relation_args.master.trim()).unwrap();
            let target = LunisDateTime::from_rfc3339(relation_args.target.trim()).unwrap();
            let lang = &relation_args.fmt.lang;
            let (_, master_stem, _) = master.get_day();

            if relation_args.all {
                let all = TenGod::resolve_all(master, target);
                print_pillar_gods(&all, lang);

                if relation_args.nayin {
                    print_nayin(&all, lang);
                }
                return;
            }

            let pillar = &relation_args.pillar;
            let pillar_data = get_pillar_data(&target, pillar);
            let pt = TenGod::resolve_pillar(master_stem, pillar_data);

            if relation_args.nayin {
                let nayin = NaYin::from_sexagenary(pt.stem, pt.branch);
                let nayin_name = nayin.to_str(lang);
                let element_name = nayin.element.to_str(lang);
                let god_name = pt.stem_god.to_str(lang);
                let god_desc = pt.stem_god.describe(lang);
                match relation_args.fmt.output {
                    OutputMode::Default => {
                        println!(
                            "{}: {}  | {} ({}) ({})",
                            god_name,
                            god_desc,
                            nayin_name,
                            element_name,
                            pt.stem.to_str(lang)
                        );
                    }
                    OutputMode::Waybar => {
                        let output = WaybarOutput {
                            text: god_name.to_string(),
                            alt: god_name.to_string(),
                            tooltip: format!(
                                "{}: {}  | {} ({})",
                                god_name, god_desc, nayin_name, element_name
                            ),
                            class: Some("lunis".to_string()),
                        };
                        println!("{}", serde_json::to_string(&output).unwrap());
                    }
                }
                return;
            }

            let line = format_stem_gods(pt.stem, pt.stem_god, &pt.hidden_gods, lang);
            let desc = pt.stem_god.describe(lang);

            match relation_args.fmt.output {
                OutputMode::Default => println!("{} — {}", line, desc),
                OutputMode::Waybar => {
                    let output = WaybarOutput {
                        text: pt.stem_god.to_str(lang).to_string(),
                        alt: pt.stem_god.to_str(lang).to_string(),
                        tooltip: format!("{} — {}", line, desc),
                        class: Some("lunis".to_string()),
                    };
                    println!("{}", serde_json::to_string(&output).unwrap());
                }
            }
        }
    }
}
