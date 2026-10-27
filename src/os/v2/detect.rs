use super::{Arch, OS};

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

pub fn get_arch() -> Arch {
    if cfg!(target_arch = "x86_64") {
        Arch::X64
    } else if cfg!(target_arch = "x86") {
        Arch::X86
    } else if cfg!(target_arch = "aarch64") {
        Arch::Arm64
    } else if cfg!(target_arch = "arm") {
        Arch::Arm
    } else {
        Arch::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_not_unknown_on_supported() {
        if cfg!(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows"
        )) {
            assert_ne!(get_os(), OS::Unknown);
        }
    }

    #[test]
    fn arch_not_unknown_on_common() {
        if cfg!(any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "aarch64",
            target_arch = "arm"
        )) {
            assert_ne!(get_arch(), Arch::Unknown);
        }
    }

    #[test]
    fn display_matches_as_str() {
        assert_eq!(get_os().to_string(), get_os().as_str());
        assert_eq!(get_arch().to_string(), get_arch().as_str());
    }
}
