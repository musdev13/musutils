#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86,
    X64,
    Arm,
    Arm64,
    Unknown,
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
