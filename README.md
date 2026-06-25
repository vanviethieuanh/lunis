# Lunis

**Lunis** is a command-line tool that converts Gregorian dates to the Chinese, Vietnamese, Japanese, and Korean lunisolar calendar. It supports sexagenary cycles (stems, branches, yinyang, wuxing), TenGod relationships, NaYin attributes, leap months, 15-minute ke divisions, and day auspiciousness scoring.

---

## Features

* **Convert** Gregorian dates (RFC 3339) to the lunisolar calendar.
* **Now** — use the current system time as input.
* **Relation** — compute TenGod (十神) interaction between two dates across all four pillars.
* **Judge** — evaluate whether a day is auspicious relative to your birth date.
* **Waybar** output mode (`-o waybar`) for desktop status bar integration.
* Multiple languages: Vietnamese (`vi`), Chinese (`zh`), Korean (`ko`), Japanese (`jp`).
* Display year, month, day, and hour with stem, branch, yinyang, wuxing, and TenGod.
* Support leap months in string format (`{m-S}`) — e.g. "Nhuận tháng 3", "闰三月".
* Show 15-minute ke divisions (`{ke}`).
* Flexible format strings for fully customizable output.
* Four-pillar hidden stems (藏干), NaYin (纳音) attributes, and seasonal strength (旺相休囚死).
* Day Officer system (建除十二神) and branch relationships (clash, harm, penalty, harmony, triad).

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

### Check TenGod relation between two dates

```bash
lunis relation -l vi "2000-01-01T12:00:00+07:00" "2026-03-02T00:59:33+07:00"
```

### Show all four pillars with hidden stems

```bash
lunis relation --all -l vi "2000-01-01T12:00:00+07:00" "2026-03-02T00:59:33+07:00"
```

### Show NaYin attribute

```bash
lunis relation --nayin -l zh "2000-01-01T12:00:00+07:00" "2026-03-02T00:59:33+07:00"
```

### Judge day auspiciousness

```bash
lunis judge -l vi -b "2000-01-01T12:00:00+07:00"
# default target is today; specify a specific day with -t
lunis judge -l zh -b "2000-01-01T12:00:00+07:00" -t "2026-03-02T00:59:33+07:00"
```

### Waybar output

```bash
lunis now -o waybar -l vi
# {"text":"...","alt":"...","tooltip":"...","class":"lunis"}
```

---

## Format String Placeholders

| Placeholder | Field | Description |
|-------------|-------|-------------|
| `{y-s}` | Year | Heavenly Stem |
| `{y-b}` | Year | Earthly Branch |
| `{y-y}` | Year | Yin/Yang |
| `{y-w}` | Year | WuXing (five phases) |
| `{y-n}` | Year | Year number |
| `{m-s}` | Month | Heavenly Stem |
| `{m-b}` | Month | Earthly Branch |
| `{m-y}` | Month | Yin/Yang |
| `{m-w}` | Month | WuXing |
| `{m-n}` | Month | Month number (1–12) |
| `{m-S}` | Month | Localized month name (e.g. 正月, Tháng Giêng) |
| `{d-s}` | Day | Heavenly Stem |
| `{d-b}` | Day | Earthly Branch |
| `{d-y}` | Day | Yin/Yang |
| `{d-w}` | Day | WuXing |
| `{d-n}` | Day | Day number (1–30) |
| `{h-s}` | Hour | Heavenly Stem |
| `{h-b}` | Hour | Earthly Branch |
| `{h-y}` | Hour | Yin/Yang |
| `{h-w}` | Hour | WuXing |
| `{h-n}` | Hour | Hour number (0–23) |
| `{ke}` | Ke | 15-minute ke division (0–7) |

---

## Options

All subcommands share these options:

| Flag | Description |
|------|-------------|
| `-l, --lang <LANG>` | Language: `vi`, `zh`, `ko`, `jp` |
| `-f, --format <FORMAT>` | Format string with placeholders |
| `-o, --output <MODE>` | Output mode: `default` (plain text) or `waybar` (JSON) |
| `--tooltip-format <FORMAT>` | Separate format for Waybar tooltip field |

### `relation` subcommand

| Flag | Description |
|------|-------------|
| `-p, --pillar <PILLAR>` | Target pillar: `year`, `month`, `day` (default), `hour` |
| `-a, --all` | Show all four pillars with hidden stems (overrides `--pillar`) |
| `--nayin` | Show NaYin (纳音) attribute |

### `judge` subcommand

| Flag | Description |
|------|-------------|
| `-b, --born <RFC3339>` | (Required) Birth date |
| `-t, --target <RFC3339>` | Target date to evaluate (defaults to now) |

---

## Supported Languages

| Code | Language | Examples |
|------|----------|---------|
| `vi` | Vietnamese | Giáp, Ất, Bính, Tý, Sửu, Dần |
| `zh` | Chinese | 甲, 乙, 丙, 子, 丑, 寅 |
| `ko` | Korean | 갑, 을, 병, 자, 축, 인 |
| `jp` | Japanese | こう, おつ, へい, し, ちゅう, いん |

---

## Notes

* Only supports dates from **31 Jan 1900** to **31 Dec 2099**.
* Timezones are ignored; all calculations are based on the provided datetime values.

---

## See Also

* `man lunis` — man page with full reference.
* `Taskfile.yml` — convenience tasks (`task build`, `task judge`, `task now`, etc.).
