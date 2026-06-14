use crate::{datetime::LunisDateTime, lang::LunarLang};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn generates(a: WuXing, b: WuXing) -> bool {
        matches!(
            (a, b),
            (WuXing::Wood, WuXing::Fire)
                | (WuXing::Fire, WuXing::Earth)
                | (WuXing::Earth, WuXing::Metal)
                | (WuXing::Metal, WuXing::Water)
                | (WuXing::Water, WuXing::Wood)
        )
    }

    fn controls(a: WuXing, b: WuXing) -> bool {
        matches!(
            (a, b),
            (WuXing::Wood, WuXing::Earth)
                | (WuXing::Earth, WuXing::Water)
                | (WuXing::Water, WuXing::Fire)
                | (WuXing::Fire, WuXing::Metal)
                | (WuXing::Metal, WuXing::Wood)
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn get_yinyang(&self) -> YinYang {
        match self {
            Branch::Zi | Branch::Yin | Branch::Chen | Branch::Wu | Branch::Shen | Branch::Xu => {
                YinYang::Yang
            }
            Branch::Chou
            | Branch::Mao
            | Branch::Si
            | Branch::Wei
            | Branch::You
            | Branch::Hai => YinYang::Yin,
        }
    }

    pub fn get_wuxing(&self) -> WuXing {
        match self {
            Branch::Zi | Branch::Hai => WuXing::Water,
            Branch::Yin | Branch::Mao => WuXing::Wood,
            Branch::Si | Branch::Wu => WuXing::Fire,
            Branch::Shen | Branch::You => WuXing::Metal,
            Branch::Chou | Branch::Chen | Branch::Wei | Branch::Xu => WuXing::Earth,
        }
    }

    pub fn get_hidden_stems(&self) -> &[Stem] {
        use Stem::*;
        match self {
            Branch::Zi => &[Gui],
            Branch::Chou => &[Ji, Xin, Gui],
            Branch::Yin => &[Jia, Bing, Wu],
            Branch::Mao => &[Yi],
            Branch::Chen => &[Wu, Yi, Gui],
            Branch::Si => &[Bing, Wu, Geng],
            Branch::Wu => &[Ding, Ji],
            Branch::Wei => &[Ji, Ding, Yi],
            Branch::Shen => &[Geng, Ren, Wu],
            Branch::You => &[Xin],
            Branch::Xu => &[Wu, Xin, Ding],
            Branch::Hai => &[Ren, Jia],
        }
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenGod {
    BiJian,
    JieCai,
    ShiShen,
    ShangGuan,
    ZhengCai,
    PianCai,
    ZhengGuan,
    QiSha,
    ZhengYin,
    PianYin,
}

impl TenGod {
    pub fn resolve(master_stem: Stem, date_stem: Stem) -> TenGod {
        let relation = TenGodRelation::resolve_relation_stems(master_stem, date_stem);

        let m = master_stem.get_yinyang();
        let d = date_stem.get_yinyang();

        match (relation, m == d) {
            (TenGodRelation::SameElement, true) => TenGod::BiJian,
            (TenGodRelation::SameElement, false) => TenGod::JieCai,

            (TenGodRelation::GeneratesMe, true) => TenGod::ZhengYin,
            (TenGodRelation::GeneratesMe, false) => TenGod::PianYin,

            (TenGodRelation::IGenerate, true) => TenGod::ShangGuan,
            (TenGodRelation::IGenerate, false) => TenGod::ShiShen,

            (TenGodRelation::IControl, true) => TenGod::ZhengCai,
            (TenGodRelation::IControl, false) => TenGod::PianCai,

            (TenGodRelation::ControlsMe, true) => TenGod::ZhengGuan,
            (TenGodRelation::ControlsMe, false) => TenGod::QiSha,
        }
    }

    pub fn resolve_tengod(master: LunisDateTime, date: LunisDateTime) -> TenGod {
        let (_, master_stem, _) = master.get_day();
        let (_, date_stem, _) = date.get_day();

        Self::resolve(master_stem, date_stem)
    }

    pub fn to_str(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => [
                "Tỷ Kiên",
                "Kiếp Tài",
                "Thực Thần",
                "Thương Quan",
                "Chính Tài",
                "Thiên Tài",
                "Chính Quan",
                "Thất Sát",
                "Chính Ấn",
                "Thiên Ấn",
            ],
            LunarLang::Zh => [
                "比肩", "劫财", "食神", "伤官", "正财", "偏财", "正官", "七杀", "正印", "偏印",
            ],
            LunarLang::Ko => [
                "비견", "겁재", "식신", "상관", "정재", "편재", "정관", "칠살", "정인", "편인",
            ],
            LunarLang::Jp => [
                "比肩", "劫財", "食神", "傷官", "正財", "偏財", "正官", "七殺", "正印", "偏印",
            ],
        };

        table[self as usize]
    }

    pub fn describe(self, lang: &LunarLang) -> &'static str {
        let table = match lang {
            LunarLang::Vi => [
                "Người cùng hành với bạn, giúp đỡ bạn và cạnh tranh cùng bạn",
                "Người cùng hành nhưng khác âm dương, đôi khi gây tranh chấp, cạnh tranh",
                "Sinh ra tài năng, năng lực, mang lại may mắn, thuận lợi trong học tập, sáng tạo",
                "Năng lực sáng tạo nhưng có thể chống đối, thể hiện cá tính, thách thức",
                "Tài chính chính thức, tiền bạc chính đáng, hợp tác rõ ràng",
                "Tài chính phụ, tiền bạc ngoài lề hoặc cơ hội tài lộc bất ngờ",
                "Quan chức, quyền lực, kỷ luật, trách nhiệm, luật pháp",
                "Thất sát, quyền lực mạnh nhưng áp lực, thử thách, cạnh tranh khốc liệt",
                "Chính ấn, sự trợ giúp, kiến thức, học tập, nghiên cứu",
                "Thiên ấn, trợ giúp gián tiếp, học hỏi từ môi trường, người khác",
            ],
            LunarLang::Zh => [
                "比肩，同类帮扶与竞争",
                "劫财，同类异性，可能引发争执或竞争",
                "食神，生才华、好运，利于学习创造",
                "伤官，创造力但可能对抗，展现个性",
                "正财，正当财务，明确合作",
                "偏财，额外财运或机会",
                "正官，权力、责任、法纪",
                "七杀，权势强但有压力和挑战",
                "正印，助力、知识、学习研究",
                "偏印，间接帮助，从环境或他人学习",
            ],
            LunarLang::Ko => [
                "비견, 동류로 도움과 경쟁",
                "겁재, 동류지만 음양 다름, 때때로 갈등 발생",
                "식신, 재능과 행운을 주며 학습과 창작에 유리",
                "상관, 창의력 있지만 대립 가능, 개성 표현",
                "정재, 정식 재물, 명확한 협력",
                "편재, 추가 재물이나 예상치 못한 기회",
                "정관, 권력, 책임, 법규",
                "칠살, 강한 권력이나 압력과 도전",
                "정인, 지원, 지식, 학습과 연구",
                "편인, 간접 지원, 환경이나 타인으로부터 학습",
            ],
            LunarLang::Jp => [
                "比肩、同じ属性の人との助け合いと競争",
                "劫財、同じ属性だが陰陽が異なる、時に争いや競争を生む",
                "食神、才能や幸運を生み、学習・創造に有利",
                "傷官、創造力だが対抗する可能性、個性を表す",
                "正財、正当な財、明確な協力",
                "偏財、追加の財運や予期しない機会",
                "正官、権力、責任、法規",
                "七殺、強い権力だがプレッシャーや挑戦あり",
                "正印、支援、知識、学習・研究",
                "偏印、間接的支援、環境や他人から学ぶ",
            ],
        };

        table[self as usize]
    }
}

#[derive(Debug, Clone)]
pub struct PillarTenGod {
    pub number: u32,
    pub stem: Stem,
    pub branch: Branch,
    pub stem_god: TenGod,
    pub hidden_gods: Vec<(Stem, TenGod)>,
}

#[derive(Debug, Clone)]
pub struct PillarGods {
    pub year: PillarTenGod,
    pub month: PillarTenGod,
    pub day: PillarTenGod,
    pub hour: PillarTenGod,
}

impl TenGod {
    pub fn resolve_pillar(master_stem: Stem, (number, stem, branch): (u32, Stem, Branch)) -> PillarTenGod {
        let stem_god = Self::resolve(master_stem, stem);
        let hidden_gods = branch
            .get_hidden_stems()
            .iter()
            .map(|&hs| (hs, Self::resolve(master_stem, hs)))
            .collect();

        PillarTenGod {
            number,
            stem,
            branch,
            stem_god,
            hidden_gods,
        }
    }

    pub fn resolve_all(master: LunisDateTime, target: LunisDateTime) -> PillarGods {
        let (_, master_stem, _) = master.get_day();

        PillarGods {
            year: Self::resolve_pillar(master_stem, target.get_year()),
            month: Self::resolve_pillar(master_stem, target.get_month()),
            day: Self::resolve_pillar(master_stem, target.get_day()),
            hour: Self::resolve_pillar(master_stem, target.get_hour()),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TenGodRelation {
    SameElement,
    GeneratesMe,
    IGenerate,
    IControl,
    ControlsMe,
}

impl TenGodRelation {
    pub fn resolve_relation_stems(master_stem: Stem, date_stem: Stem) -> TenGodRelation {
        let m = master_stem.get_wuxing();
        let d = date_stem.get_wuxing();

        if m == d {
            TenGodRelation::SameElement
        } else if WuXing::generates(d, m) {
            TenGodRelation::GeneratesMe
        } else if WuXing::generates(m, d) {
            TenGodRelation::IGenerate
        } else if WuXing::controls(m, d) {
            TenGodRelation::IControl
        } else if WuXing::controls(d, m) {
            TenGodRelation::ControlsMe
        } else {
            unreachable!("Invalid WuXing relationship");
        }
    }

    pub fn resolve_relation(master: LunisDateTime, date: LunisDateTime) -> TenGodRelation {
        let (_, master_stem, _) = master.get_day();
        let (_, date_stem, _) = date.get_day();

        Self::resolve_relation_stems(master_stem, date_stem)
    }
}
