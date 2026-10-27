#[cfg(all(feature = "os-v1", feature = "os-v2"))]
compile_error!("features `os-v1` and `os-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "os-v1")]
pub mod v1;
#[cfg(feature = "os-v2")]
pub mod v2;

#[cfg(feature = "os-v1")]
pub use self::v1::*;
#[cfg(all(feature = "os-v2", not(feature = "os-v1")))]
pub use self::v2::*;
