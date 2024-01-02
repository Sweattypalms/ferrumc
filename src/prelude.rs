pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Implement external traits for external types
pub struct W<T>(pub T);

pub use std::format as f;