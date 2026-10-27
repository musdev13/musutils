#[cfg(all(feature = "types-v1", feature = "types-v2"))]
compile_error!("features `types-v1` and `types-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "types-v1")]
pub mod v1;
#[cfg(feature = "types-v2")]
pub mod v2;

#[cfg(feature = "types-v1")]
pub use self::v1::*;
#[cfg(all(feature = "types-v2", not(feature = "types-v1")))]
pub use self::v2::*;
