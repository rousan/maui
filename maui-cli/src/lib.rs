pub use self::error::{Error, ErrorExt, ResultExt};

pub mod constants;
mod error;
pub mod file_size;
pub mod prelude;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;
