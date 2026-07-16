pub enum OS {
    Windows,
    Linux,
    MacOs,
    Unknown,
}

pub fn get_os() -> OS {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "linux") {
        OS::Linux
    } else if cfg!(target_os = "macos") {
        OS::MacOs
    } else {
        OS::Unknown
    }
}
