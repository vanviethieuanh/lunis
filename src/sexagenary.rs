use crate::lang::LunarLang;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum WuXing {
    Metal,
    Water,
    Wood,
    Fire,
    Earth,
}

impl WuXing {
    pub fn to_str(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => ["Kim", "Thủy", "Mộc", "Hỏa", "Thổ"],
            LunarLang::Zh => ["金", "水", "木", "火", "土"],
            LunarLang::Ko => ["금", "수", "목", "화", "토"],
            LunarLang::Jp => ["きん", "すい", "もく", "か", "ど"],
        };

        table[self as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum YinYang {
    Yin,
    Yang,
}

impl YinYang {
    #[inline]
    pub fn to_str(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => ["Âm", "Dương"],
            LunarLang::Zh => ["阴", "阳"],
            LunarLang::Ko => ["음", "양"],
            LunarLang::Jp => ["いん", "よう"],
        };

        table[self as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Stem {
    Jia,  // 甲
    Yi,   // 乙
    Bing, // 丙
    Ding, // 丁
    Wu,   // 戊
    Ji,   // 己
    Geng, // 庚
    Xin,  // 辛
    Ren,  // 壬
    Gui,  // 癸
}

impl Stem {
    #[inline]
    pub fn to_str(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => [
                "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý",
            ],
            LunarLang::Zh => ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"],
            LunarLang::Ko => ["갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"],
            LunarLang::Jp => [
                "こう", "おつ", "へい", "てい", "ぼ", "き", "こう", "しん", "じん", "き",
            ],
        };

        table[self as usize]
    }

    pub fn get_yinyang(&self) -> YinYang {
        match self {
            Stem::Jia | Stem::Bing | Stem::Wu | Stem::Geng | Stem::Ren => YinYang::Yang,
            Stem::Yi | Stem::Ding | Stem::Ji | Stem::Xin | Stem::Gui => YinYang::Yin,
        }
    }

    pub fn get_wuxing(&self) -> WuXing {
        match self {
            Stem::Jia | Stem::Yi => WuXing::Wood,
            Stem::Bing | Stem::Ding => WuXing::Fire,
            Stem::Wu | Stem::Ji => WuXing::Earth,
            Stem::Geng | Stem::Xin => WuXing::Metal,
            Stem::Ren | Stem::Gui => WuXing::Water,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Branch {
    Zi,   // 子
    Chou, // 丑
    Yin,  // 寅
    Mao,  // 卯
    Chen, // 辰
    Si,   // 巳
    Wu,   // 午
    Wei,  // 未
    Shen, // 申
    You,  // 酉
    Xu,   // 戌
    Hai,  // 亥
}

impl Branch {
    #[inline]
    pub fn to_str(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => [
                "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
            ],
            LunarLang::Zh => [
                "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
            ],
            LunarLang::Ko => [
                "자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해",
            ],
            LunarLang::Jp => [
                "し",
                "ちゅう",
                "いん",
                "ぼう",
                "しん",
                "み",
                "ご",
                "び",
                "しん",
                "ゆう",
                "じゅつ",
                "かい",
            ],
        };

        table[self as usize]
    }
}

impl From<u32> for Stem {
    fn from(v: u32) -> Self {
        match v {
            0 => Stem::Jia,
            1 => Stem::Yi,
            2 => Stem::Bing,
            3 => Stem::Ding,
            4 => Stem::Wu,
            5 => Stem::Ji,
            6 => Stem::Geng,
            7 => Stem::Xin,
            8 => Stem::Ren,
            9 => Stem::Gui,
            _ => panic!("invalid stem index"),
        }
    }
}

impl From<u32> for Branch {
    fn from(v: u32) -> Self {
        match v {
            0 => Branch::Zi,
            1 => Branch::Chou,
            2 => Branch::Yin,
            3 => Branch::Mao,
            4 => Branch::Chen,
            5 => Branch::Si,
            6 => Branch::Wu,
            7 => Branch::Wei,
            8 => Branch::Shen,
            9 => Branch::You,
            10 => Branch::Xu,
            11 => Branch::Hai,
            _ => panic!("invalid branch index"),
        }
    }
}
