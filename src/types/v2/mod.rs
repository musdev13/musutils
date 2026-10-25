mod deoption;
pub mod line;
mod status;

#[allow(deprecated)]
pub use deoption::deoption;
pub use line::{draw, draw_colored};
pub use status::Status;
