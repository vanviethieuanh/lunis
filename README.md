# Lunis CLI

**Lunis** is a command-line tool that converts Gregorian dates to the Chinese, Vietnamese, Japanese, and Korean lunar calendar. It supports sexagenary cycles, stems, branches, yinyang, wuxing, leap months, and 15-minute ke divisions. You can customize the output using flexible format strings.

---

## Features

* Convert a Gregorian date to the lunar calendar.
* Support for multiple languages: Vietnamese (vi), Chinese (zh), Korean (ko), Japanese (jp).
* Display year, month, day, and hour with stem, branch, yinyang, and wuxing.
* Support leap months in string format (`{m-S}`) like "Nhuận tháng 3" or "闰三月".
* Show 15-minute ke divisions.
* Flexible format strings for fully customizable output.
* Modes:

  * `convert`: Reads a date from standard input or as an argument in RFC 3339 format.
  * `now`: Uses the current system time as input.

---

## Installation

```bash
cargo install --path .
```

---

## Usage

### Convert a date

```bash
echo "2026-03-02T00:59:33+07:00" | lunis convert -l vi -f '{y-s} {y-b} - {m-s} {m-b} ({m-S}) - {d-s} {d-b} | Giờ {h-b} {ke} khắc'
# Bính Ngọ - Canh Dần (Tháng Giêng) - Ất Hợi | Giờ Tý 7 khắc
```

### Use current time

```bash
lunis now -l zh -f "{y-s}{y-b}年 {m-S} {d-s}{d-b}日"
# 乙巳年 十一月 丁丑日
```

---

## Format String Placeholders

* **Year**: `{y-s}`, `{y-b}`, `{y-y}`, `{y-w}`, `{y-n}`
* **Month**: `{m-s}`, `{m-b}`, `{m-y}`, `{m-w}`, `{m-n}`, `{m-S}`
* **Day**: `{d-s}`, `{d-b}`, `{d-y}`, `{d-w}`, `{d-n}`
* **Hour**: `{h-s}`, `{h-b}`, `{h-y}`, `{h-w}`, `{h-n}`
* **Ke**: `{ke}`

---

## Notes

* Only support time in range `31 Jan 1900` - `31 Dec 2099`
* Timezones are ignored; all calculations are based on the provided datetime values.
