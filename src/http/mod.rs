#[cfg(all(feature = "http-v1", feature = "http-v2"))]
compile_error!("features `http-v1` and `http-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "http-v1")]
pub mod v1;
#[cfg(feature = "http-v2")]
pub mod v2;

#[cfg(feature = "http-v1")]
pub use self::v1::*;
#[cfg(all(feature = "http-v2", not(feature = "http-v1")))]
pub use self::v2::*;
