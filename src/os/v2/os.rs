use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OS {
    Windows,
    Linux,
    MacOs,
    Unknown,
}

impl OS {
    pub fn as_str(&self) -> &'static str {
        match self {
            OS::Windows => "windows",
            OS::Linux => "linux",
            OS::MacOs => "macos",
            OS::Unknown => "unknown",
        }
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
