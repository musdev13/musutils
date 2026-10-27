#[cfg(all(feature = "vec-v1", feature = "vec-v2"))]
compile_error!("features `vec-v1` and `vec-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "vec-v1")]
pub mod v1;
#[cfg(feature = "vec-v2")]
pub mod v2;

#[cfg(feature = "vec-v1")]
pub use self::v1::*;
#[cfg(all(feature = "vec-v2", not(feature = "vec-v1")))]
pub use self::v2::*;
