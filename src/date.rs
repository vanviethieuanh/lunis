use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;

use crate::datetime::Error;

// Source: https://github.com/messense/rust-lunardate/blob/main/src/lib.rs
lazy_static! {
    pub static ref START_DATE: NaiveDate = NaiveDate::from_ymd_opt(1900, 1, 31).unwrap();
    pub static ref YEAR_DAYS: Vec<u32> = {
        let mut days = Vec::with_capacity(YEAR_INFOS.len());
        for year_info in &YEAR_INFOS {
            days.push(year_into_days(*year_info));
        }
        days
    };
}

pub const YEAR_INFOS: [u32; 200] = [
    /* encoding:
                b bbbbbbbbbbbb bbbb
        bit#    1 111111000000 0000
                6 543210987654 3210
                . ............ ....
        month#    000000000111
                M 123456789012   L

    b_j = 1 for long month, b_j = 0 for short month
    L is the leap month of the year if 1<=L<=12; NO leap month if L = 0.
    The leap month (if exists) is long one iff M = 1.
    */
    0x04bd8, /* 1900 */
    0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, /* 1905 */
    0x16554, 0x056a0, 0x09ad0, 0x055d2, 0x04ae0, /* 1910 */
    0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, /* 1915 */
    0x0d6a0, 0x0ada2, 0x095b0, 0x14977, 0x04970, /* 1920 */
    0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, /* 1925 */
    0x02b60, 0x09570, 0x052f2, 0x04970, 0x06566, /* 1930 */
    0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60, /* 1935 */
    0x186e3, 0x092e0, 0x1c8d7, 0x0c950, 0x0d4a0, /* 1940 */
    0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, /* 1945 */
    0x092d0, 0x0d2b2, 0x0a950, 0x0b557, 0x06ca0, /* 1950 */
    0x0b550, 0x15355, 0x04da0, 0x0a5d0, 0x14573, /* 1955 */
    0x052d0, 0x0a9a8, 0x0e950, 0x06aa0, 0x0aea6, /* 1960 */
    0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, /* 1965 */
    0x0f263, 0x0d950, 0x05b57, 0x056a0, 0x096d0, /* 1970 */
    0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, /* 1975 */
    0x0d558, 0x0b540, 0x0b5a0, 0x195a6, 0x095b0, /* 1980 */
    0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, /* 1985 */
    0x06d40, 0x0af46, 0x0ab60, 0x09570, 0x04af5, /* 1990 */
    0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, /* 1995 */
    0x05ac0, 0x0ab60, 0x096d5, 0x092e0, 0x0c960, /* 2000 */
    0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, /* 2005 */
    0x0abb7, 0x025d0, 0x092d0, 0x0cab5, 0x0a950, /* 2010 */
    0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, /* 2015 */
    0x0a5b0, 0x15176, 0x052b0, 0x0a930, 0x07954, /* 2020 */
    0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, /* 2025 */
    0x0a4e0, 0x0d260, 0x0ea65, 0x0d530, 0x05aa0, /* 2030 */
    0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0, /* 2035 */
    0x1d0b6, 0x0d250, 0x0d520, 0x0dd45, 0x0b5a0, /* 2040 */
    0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, /* 2045 */
    0x0aa50, 0x1b255, 0x06d20, 0x0ada0, 0x14b63, /* 2050 */
    0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, /* 2055 */
    0x0ea50, 0x06aa0, 0x1a6c4, 0x0aae0, 0x092e0, /* 2060 */
    0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, /* 2065 */
    0x05d55, 0x056a0, 0x0a6d0, 0x055d4, 0x052d0, /* 2070 */
    0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, /* 2075 */
    0x055a0, 0x0aba4, 0x0a5b0, 0x052b0, 0x0b273, /* 2080 */
    0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55, /* 2085 */
    0x04b60, 0x0a570, 0x054e4, 0x0d160, 0x0e968, /* 2090 */
    0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, /* 2095 */
    0x0a9d4, 0x0a2d0, 0x0d150, 0x0f252, /* 2099 */
];

pub fn resolve_date(year_info: u32, day_of_year: u32) -> Result<(u32, u32, bool), Error> {
    let leap_month = year_info & 0xF;
    let has_leap = leap_month != 0;

    let mut doy = day_of_year;

    for month in 1..=12 {
        let days = 29 + (year_info >> (16 - month) & 1);
        if doy < days {
            return Ok((month, doy + 1, false));
        }

        doy -= days;

        if has_leap && month == leap_month {
            let l_days = 29 + ((year_info >> 16) & 1);

            if doy < l_days {
                return Ok((month, doy + 1, true));
            }

            doy -= l_days;
        }
    }

    Err(Error::OutOfRange)
}

pub fn naive_jdn(n: &NaiveDate) -> i64 {
    gregorian_to_jdn(n.year(), n.month(), n.day())
}

pub fn to_chinese_number(n: u32) -> &'static str {
    match n {
        1 => "一",
        2 => "二",
        3 => "三",
        4 => "四",
        5 => "五",
        6 => "六",
        7 => "七",
        8 => "八",
        9 => "九",
        10 => "十",
        11 => "十一",
        12 => "十二",
        _ => "?",
    }
}

fn year_into_days(year_info: u32) -> u32 {
    let mut r = 12 * 29u32;

    r += (year_info & 0xFFF0).count_ones();

    let has_leap = (year_info & 0xF != 0) as u32;
    let long_leap = ((year_info >> 16) & 1) as u32;

    r += has_leap * (29 + long_leap);

    return r;
}

fn gregorian_to_jdn(year: i32, month: u32, day: u32) -> i64 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;

    day as i64 + ((153 * m + 2) / 5) as i64 + (365 * y) as i64 + (y / 4) as i64 - (y / 100) as i64
        + (y / 400) as i64
        - 32045
}
