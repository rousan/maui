pub use self::error::{Error, ErrorExt, ResultExt};

pub mod constants;
mod error;
pub mod help;
pub mod prelude;
pub mod tunnel;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;
