pub mod error;
pub use error::Error;
pub use error::ErrorKind;
pub type Result<T> = std::result::Result<T, Error>;
