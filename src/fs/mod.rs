#[cfg(all(feature = "fs-v1", feature = "fs-v2"))]
compile_error!("features `fs-v1` and `fs-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "fs-v1")]
pub mod v1;
#[cfg(feature = "fs-v2")]
pub mod v2;

#[cfg(feature = "fs-v1")]
pub use self::v1::*;
#[cfg(all(feature = "fs-v2", not(feature = "fs-v1")))]
pub use self::v2::*;
