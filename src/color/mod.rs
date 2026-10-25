#[cfg(all(feature = "color-v1", feature = "color-v2"))]
compile_error!("features `color-v1` and `color-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "color-v1")]
pub mod v1;
#[cfg(feature = "color-v2")]
pub mod v2;

#[cfg(feature = "color-v1")]
pub use self::v1::*;
#[cfg(all(feature = "color-v2", not(feature = "color-v1")))]
pub use self::v2::*;
