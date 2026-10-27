use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X86,
    X64,
    Arm,
    Arm64,
    Unknown,
}

impl Arch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Arch::X86 => "x86",
            Arch::X64 => "x86_64",
            Arch::Arm => "arm",
            Arch::Arm64 => "aarch64",
            Arch::Unknown => "unknown",
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
