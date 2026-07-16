pub enum Colors{
    Green,
    Yellow,
    Blue,
    Lime,
    Gold,
    Cyan,
    Red,
    Orange,
    Purple,
    Rose,
    Gray,
    Reset
}

impl Colors {
    pub fn as_term_code(&self) -> &'static str {
        match self {
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Lime => "\x1b[92m",
            Colors::Gold => "\x1b[38;5;220m",
            Colors::Cyan => "\x1b[36m",
            Colors::Red => "\x1b[31m",
            Colors::Orange => "\x1b[38;5;208m",
            Colors::Purple => "\x1b[35m",
            Colors::Rose => "\x1b[95m",
            Colors::Gray => "\x1b[90m",
            Colors::Reset => "\x1b[0m"
        }
    }
}
