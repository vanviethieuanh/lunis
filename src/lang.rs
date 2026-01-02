#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarLang {
    Vi, // Vietnamese
    Zh, // Chinese
    Ko, // Korean
    Jp, // Japanese (Onyomi)
}

impl std::str::FromStr for LunarLang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vi" => Ok(LunarLang::Vi),
            "zh" => Ok(LunarLang::Zh),
            "ko" => Ok(LunarLang::Ko),
            "jp" => Ok(LunarLang::Jp),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}

impl std::fmt::Display for LunarLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LunarLang::Vi => "vi",
            LunarLang::Zh => "zh",
            LunarLang::Ko => "ko",
            LunarLang::Jp => "jp",
        };
        write!(f, "{}", s)
    }
}
