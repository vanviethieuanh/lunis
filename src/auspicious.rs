use crate::{
    datetime::LunisDateTime,
    lang::LunarLang,
    sexagenary::{Branch, NaYin, TenGod, WuXing},
};

pub struct DayRating {
    pub score: u32,
    pub label: String,
    pub detail: String,
}

fn tengod_kind(god: TenGod, lang: &LunarLang) -> &'static str {
    let table = match lang {
        LunarLang::Vi => [
            "Trung", "Hung", "Cát", "Hung", "Cát", "Trung", "Cát", "Hung", "Cát", "Hung",
        ],
        LunarLang::Zh => ["中", "凶", "吉", "凶", "吉", "中", "吉", "凶", "吉", "凶"],
        LunarLang::Ko => ["중", "흉", "길", "흉", "길", "중", "길", "흉", "길", "흉"],
        LunarLang::Jp => ["中", "凶", "吉", "凶", "吉", "中", "吉", "凶", "吉", "凶"],
    };
    table[god as usize]
}

fn tengod_desc(god: TenGod, lang: &LunarLang) -> &'static str {
    let table = match lang {
        LunarLang::Vi => [
            "Năng lượng đồng hành, cạnh tranh",
            "Năng lượng cạnh tranh, hao tài",
            "Năng lượng tài năng, may mắn",
            "Năng lượng sáng tạo, chống đối",
            "Năng lượng tài chính ổn định",
            "Năng lượng tài lộc bất ngờ",
            "Năng lượng quyền lực, kỷ luật",
            "Năng lượng áp lực, thử thách",
            "Năng lượng hỗ trợ, học tập",
            "Năng lượng gián tiếp, bất thường",
        ],
        LunarLang::Zh => [
            "同辈陪伴竞争的能量",
            "竞争损耗的能量",
            "才华幸运的能量",
            "创造对抗的能量",
            "稳定财富的能量",
            "意外财运的能量",
            "权力纪律的能量",
            "压迫挑战的能量",
            "支持学习的能量",
            "间接偏门的能量",
        ],
        LunarLang::Ko => [
            "동료와 경쟁의 에너지",
            "경쟁과 손실의 에너지",
            "재능과 행운의 에너지",
            "창조와 반항의 에너지",
            "안정적 재물의 에너지",
            "뜻밖의 재운의 에너지",
            "권력과 규율의 에너지",
            "압박과 도전의 에너지",
            "지원과 학습의 에너지",
            "간접적이고 불규칙한 에너지",
        ],
        LunarLang::Jp => [
            "仲間と競争のエネルギー",
            "競争と損失のエネルギー",
            "才能と幸運のエネルギー",
            "創造と反抗のエネルギー",
            "安定財産のエネルギー",
            "意外な財運のエネルギー",
            "権力と規律のエネルギー",
            "圧迫と挑戦のエネルギー",
            "支援と学習のエネルギー",
            "間接的で不安定なエネルギー",
        ],
    };
    table[god as usize]
}

fn tengod_bias(god: TenGod) -> i32 {
    match god {
        TenGod::ZhengYin => 25,
        TenGod::ZhengGuan => 20,
        TenGod::ZhengCai => 20,
        TenGod::ShiShen => 15,
        TenGod::BiJian => 5,
        TenGod::PianCai => 5,
        TenGod::PianYin => -10,
        TenGod::ShangGuan => -10,
        TenGod::JieCai => -10,
        TenGod::QiSha => -15,
    }
}

fn pillar_label(idx: usize, lang: &LunarLang) -> &'static str {
    let table = match lang {
        LunarLang::Vi => ["Năm Trụ", "Tháng Trụ", "Ngày Trụ", "Giờ Trụ"],
        LunarLang::Zh => ["年柱", "月柱", "日柱", "时柱"],
        LunarLang::Ko => ["년주", "월주", "일주", "시주"],
        LunarLang::Jp => ["年柱", "月柱", "日柱", "時柱"],
    };
    table[idx]
}

fn your_pillar(idx: usize, lang: &LunarLang) -> &'static str {
    let table = match lang {
        LunarLang::Vi => ["Năm bạn", "Tháng bạn", "Ngày bạn", "Giờ bạn"],
        LunarLang::Zh => ["你年", "你月", "你日", "你时"],
        LunarLang::Ko => ["당신 년", "당신 월", "당신 일", "당신 시"],
        LunarLang::Jp => ["あなた年", "あなた月", "あなた日", "あなた時"],
    };
    table[idx]
}

fn clash_msg(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Xung khắc với ngày mục tiêu",
        LunarLang::Zh => "冲克目标日",
        LunarLang::Ko => "목표일과 충돌",
        LunarLang::Jp => "目标日と冲克",
    }
}

fn harmony_msg(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Hợp với ngày mục tiêu",
        LunarLang::Zh => "合日",
        LunarLang::Ko => "목표일과 합",
        LunarLang::Jp => "合日",
    }
}

fn triad_msg(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Tam hợp với ngày mục tiêu",
        LunarLang::Zh => "三合日",
        LunarLang::Ko => "목표일과 삼합",
        LunarLang::Jp => "三合日",
    }
}

fn penalty_msg(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Hình với ngày mục tiêu",
        LunarLang::Zh => "刑日",
        LunarLang::Ko => "목표일과 형",
        LunarLang::Jp => "刑日",
    }
}

fn nayin_word(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Na Yin",
        LunarLang::Zh => "纳音",
        LunarLang::Ko => "납음",
        LunarLang::Jp => "納音",
    }
}

fn same_nayin(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Na Yin đồng hành",
        LunarLang::Zh => "纳音相同",
        LunarLang::Ko => "납음 동일",
        LunarLang::Jp => "納音同じ",
    }
}

fn generate_word(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "sinh cho",
        LunarLang::Zh => "生",
        LunarLang::Ko => "생",
        LunarLang::Jp => "生",
    }
}

fn control_word(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "khắc",
        LunarLang::Zh => "克",
        LunarLang::Ko => "극",
        LunarLang::Jp => "克",
    }
}

fn branch_generates(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Chi sinh bạn",
        LunarLang::Zh => "支生你",
        LunarLang::Ko => "지지가 당신을 생함",
        LunarLang::Jp => "支があなたを生む",
    }
}

fn branch_controls(lang: &LunarLang) -> &'static str {
    match lang {
        LunarLang::Vi => "Chi khắc bạn",
        LunarLang::Zh => "支克你",
        LunarLang::Ko => "지지가 당신을 극함",
        LunarLang::Jp => "支があなたを克つ",
    }
}

fn rating_label(score: u32, lang: &LunarLang) -> &'static str {
    if score >= 80 {
        match lang {
            LunarLang::Vi => "Ngày Tốt",
            LunarLang::Zh => "吉日",
            LunarLang::Ko => "좋은 날",
            LunarLang::Jp => "吉日",
        }
    } else if score >= 60 {
        match lang {
            LunarLang::Vi => "Ngày Khá",
            LunarLang::Zh => "平日",
            LunarLang::Ko => "괜찮은 날",
            LunarLang::Jp => "まずまずの日",
        }
    } else if score >= 40 {
        match lang {
            LunarLang::Vi => "Ngày Trung Bình",
            LunarLang::Zh => "平日",
            LunarLang::Ko => "보통 날",
            LunarLang::Jp => "普通の日",
        }
    } else if score >= 20 {
        match lang {
            LunarLang::Vi => "Ngày Xấu",
            LunarLang::Zh => "凶日",
            LunarLang::Ko => "나쁜 날",
            LunarLang::Jp => "凶日",
        }
    } else {
        match lang {
            LunarLang::Vi => "Ngày Rất Xấu",
            LunarLang::Zh => "大凶日",
            LunarLang::Ko => "매우 나쁜 날",
            LunarLang::Jp => "大凶日",
        }
    }
}

fn is_clash(a: Branch, b: Branch) -> bool {
    (a as u32 as i32 - b as u32 as i32).rem_euclid(12) == 6
}

fn is_six_harmony(a: Branch, b: Branch) -> bool {
    let pairs = [(0, 1), (2, 11), (3, 10), (4, 9), (5, 8), (6, 7)];
    let (au, bu) = (a as u32, b as u32);
    pairs.contains(&(au, bu)) || pairs.contains(&(bu, au))
}

fn is_three_harmony(a: Branch, b: Branch) -> bool {
    let triads = [(0, 4, 8), (3, 7, 11), (2, 6, 10), (1, 5, 9)];
    if a == b {
        return false;
    }
    let (au, bu) = (a as u32, b as u32);
    triads
        .iter()
        .any(|(x, y, z)| (au == *x || au == *y || au == *z) && (bu == *x || bu == *y || bu == *z))
}

fn is_penalty(a: Branch, b: Branch) -> bool {
    if a == b {
        return matches!(a, Branch::Chen | Branch::Wu | Branch::You | Branch::Hai);
    }
    let (au, bu) = (a as u32, b as u32);
    if (au == 0 && bu == 3) || (au == 3 && bu == 0) {
        return true;
    }
    let ruthless = [2u32, 5, 8];
    if ruthless.contains(&au) && ruthless.contains(&bu) {
        return true;
    }
    let power = [1u32, 7, 10];
    if power.contains(&au) && power.contains(&bu) {
        return true;
    }
    false
}

pub fn evaluate(master: LunisDateTime, target: LunisDateTime, lang: &LunarLang) -> DayRating {
    let mut raw_score = 0i32;
    let mut lines: Vec<String> = Vec::new();

    let (_, master_stem, _) = master.get_day();
    let (_, target_day_stem, target_day_branch) = target.get_day();

    let pillar_checks = [
        target.get_year(),
        target.get_month(),
        target.get_day(),
        target.get_hour(),
    ];

    for (i, (_, stem, _)) in pillar_checks.iter().enumerate() {
        let god = TenGod::resolve(master_stem, *stem);
        let bias = tengod_bias(god);
        raw_score += bias;
        let sign = if bias >= 0 { "+" } else { "" };
        lines.push(format!(
            "  {}: {} ({}{}) {} - {}",
            pillar_label(i, lang),
            god.to_str(lang),
            sign,
            bias,
            tengod_kind(god, lang),
            tengod_desc(god, lang),
        ));
    }

    let master_branches = [
        master.get_year().2,
        master.get_month().2,
        master.get_day().2,
        master.get_hour().2,
    ];

    for (i, mb) in master_branches.iter().enumerate() {
        if is_clash(target_day_branch, *mb) {
            raw_score -= 20;
            lines.push(format!(
                "  {} {} (-20)",
                your_pillar(i, lang),
                clash_msg(lang)
            ));
        }
        if is_six_harmony(target_day_branch, *mb) {
            raw_score += 15;
            lines.push(format!(
                "  {} {} (+15)",
                your_pillar(i, lang),
                harmony_msg(lang)
            ));
        }
        if is_three_harmony(target_day_branch, *mb) {
            raw_score += 10;
            lines.push(format!(
                "  {} {} (+10)",
                your_pillar(i, lang),
                triad_msg(lang)
            ));
        }
        if is_penalty(target_day_branch, *mb) {
            raw_score -= 10;
            lines.push(format!(
                "  {} {} (-10)",
                your_pillar(i, lang),
                penalty_msg(lang)
            ));
        }
    }

    let master_nayin = NaYin::from_sexagenary(master_stem, master.get_day().2);
    let target_nayin = NaYin::from_sexagenary(target_day_stem, target_day_branch);
    if master_nayin.element == target_nayin.element {
        raw_score += 5;
        lines.push(format!(
            "  {} {} (+5)",
            same_nayin(lang),
            master_nayin.element.to_str(lang)
        ));
    } else if WuXing::generates(target_nayin.element, master_nayin.element) {
        raw_score += 10;
        lines.push(format!(
            "  {} {} {} {} (+10)",
            target_nayin.to_str(lang),
            generate_word(lang),
            nayin_word(lang),
            master_nayin.element.to_str(lang)
        ));
    } else if WuXing::controls(target_nayin.element, master_nayin.element) {
        raw_score -= 10;
        lines.push(format!(
            "  {} {} {} {} (-10)",
            target_nayin.to_str(lang),
            control_word(lang),
            nayin_word(lang),
            master_nayin.element.to_str(lang)
        ));
    }

    let branch_wuxing = target_day_branch.get_wuxing();
    let stem_wuxing = master_stem.get_wuxing();
    if WuXing::generates(branch_wuxing, stem_wuxing) {
        raw_score += 10;
        lines.push(format!("  {} (+10)", branch_generates(lang)));
    } else if WuXing::controls(branch_wuxing, stem_wuxing) {
        raw_score -= 10;
        lines.push(format!("  {} (-10)", branch_controls(lang)));
    }

    let clamped = raw_score.clamp(-100, 100);
    let score = ((clamped + 100) * 100 / 200) as u32;
    let label = rating_label(score, lang).to_string();

    DayRating {
        score,
        label,
        detail: lines.join("\n"),
    }
}
