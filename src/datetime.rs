use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Display;
use thiserror::Error;

use crate::{
    date::{START_DATE, YEAR_DAYS, YEAR_INFOS, naive_jdn, resolve_date, to_chinese_number},
    lang::LunarLang,
    sexagenary::{Branch, Stem},
};

lazy_static! {
    static ref PLACEHOLDER_RE: Regex = Regex::new(r"\{([ymdh])-([sSbywn])\}|\{ke\}").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct LunisDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,

    pub jdn: u32,

    pub leap_month: bool,
    pub time: NaiveTime,
}

impl Display for LunisDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}{}-{}",
            self.year,
            if self.leap_month { "leap " } else { "" },
            self.month,
            self.day,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error("failed to parse RFC3339 datetime: {0}")]
    ChronoParse(#[from] chrono::ParseError),

    #[error("datetime out of allow range (1900-2099)")]
    OutOfRange,
}

impl LunisDateTime {
    pub fn from_rfc3339(s: &str) -> Result<Self, Error> {
        let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(s)?;
        let n = dt.naive_local();

        Self::from_naive(&n)
    }

    pub fn now() -> Result<Self, Error> {
        let n = Local::now().naive_local();

        Self::from_naive(&n)
    }

    pub fn format_string(&self, fmt_str: &str, lang: &LunarLang) -> String {
        PLACEHOLDER_RE
            .replace_all(fmt_str, |caps: &regex::Captures| {
                if &caps[0] == "{ke}" {
                    return self.get_ke().to_string();
                }

                let field = &caps[1];
                let attr = &caps[2];

                let (number, stem, branch) = match field {
                    "y" => self.get_year(),
                    "m" => self.get_month(),
                    "d" => self.get_day(),
                    "h" => self.get_hour(),
                    _ => panic!("unknown field"),
                };

                match attr {
                    "s" => stem.to_str(lang).to_string(),
                    "b" => branch.to_str(lang).to_string(),
                    "y" => stem.get_yinyang().to_str(lang).to_string(),
                    "w" => stem.get_wuxing().to_str(lang).to_string(),
                    "n" => number.to_string(),
                    "S" => {
                        if field != "m" {
                            panic!("unknown attr")
                        }

                        return self.month_str(lang);
                    }
                    _ => panic!("unknown attr"),
                }
            })
            .to_string()
    }

    pub fn get_ke(&self) -> u32 {
        (self.time.hour() - 1) % 2 * 4 + self.time.minute() / 15
    }

    pub fn get_hour(&self) -> (u32, Stem, Branch) {
        let branch_idx = ((self.time.hour() + 1) / 2) % 12;
        let branch = Branch::from(branch_idx);

        let (_, day_stem, _) = self.get_day();

        let day_stem_idx = day_stem as u32;
        let stem_idx = (2 * day_stem_idx + branch_idx) % 10;
        let stem = Stem::from(stem_idx);

        return (self.time.hour(), stem, branch);
    }

    pub fn get_day(&self) -> (u32, Stem, Branch) {
        let idx = (self.jdn + 49) % (60);
        return (self.day, Stem::from(idx % 10), Branch::from(idx % 12));
    }

    pub fn get_month(&self) -> (u32, Stem, Branch) {
        let (_, year_stem, _) = self.get_year();

        let branch = Branch::from((self.month + 1) % 12);
        let stem = Stem::from((year_stem as u32 * 2 + self.month + 1) % 10);

        return (self.month, stem, branch);
    }

    pub fn get_year(&self) -> (u32, Stem, Branch) {
        let idx = ((self.year - 4) % (60)) as u32;
        return (
            self.year as u32,
            Stem::from(idx % 10),
            Branch::from(idx % 12),
        );
    }

    pub fn month_str(&self, lang: &LunarLang) -> String {
        match lang {
            LunarLang::Vi => {
                if self.leap_month {
                    return format!("Nhuận tháng {}", self.month);
                }
                match self.month {
                    1 => "Tháng Giêng".to_string(),
                    2..=11 => format!("Tháng {}", self.month),
                    12 => "Tháng Chạp".to_string(),
                    _ => self.month.to_string(),
                }
            }
            LunarLang::Zh => {
                let base = match self.month {
                    1 => "正月".to_string(),
                    2..=11 => format!("{}月", to_chinese_number(self.month)),
                    12 => "腊月".to_string(),
                    _ => self.month.to_string(),
                };

                if self.leap_month {
                    return format!("闰{}", base);
                }

                base
            }
            LunarLang::Ko => {
                let base = match self.month {
                    1 => "정월".to_string(),
                    2..=12 => format!("{}월", self.month),
                    _ => return self.month.to_string(),
                };
                if self.leap_month {
                    return format!("윤{}", base);
                }
                base.to_string()
            }
            LunarLang::Jp => {
                let base = match self.month {
                    1 => "正月",
                    2 => "如月",
                    3 => "弥生",
                    4 => "卯月",
                    5 => "皐月",
                    6 => "水無月",
                    7 => "文月",
                    8 => "葉月",
                    9 => "長月",
                    10 => "神無月",
                    11 => "霜月",
                    12 => "師走",
                    _ => return self.month.to_string(),
                };
                if self.leap_month {
                    return format!("閏{}", base);
                }
                base.to_string()
            }
        }
    }

    fn from_naive(n: &NaiveDateTime) -> Result<Self, Error> {
        let time = n.time();

        let offset_d = n.date().signed_duration_since(*START_DATE).num_days();
        if offset_d < 0 {
            return Err(Error::OutOfRange);
        }

        let mut offset = offset_d as u32;

        let mut year = 1900;
        for year_day in YEAR_DAYS.iter() {
            if offset < *year_day {
                break;
            }
            year += 1;
            offset -= *year_day;
        }

        let year_info = YEAR_INFOS[year - 1900];
        let (month, day, is_leap) = resolve_date(year_info, offset)?;

        Ok(LunisDateTime {
            year: year as i32,
            month: month,
            day: day,

            jdn: naive_jdn(&n.date()) as u32,

            leap_month: is_leap,
            time: time,
        })
    }
}
