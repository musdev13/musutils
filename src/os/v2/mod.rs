mod arch;
mod detect;
mod os;

pub use arch::Arch;
pub use detect::{get_arch, get_os};
pub use os::OS;
