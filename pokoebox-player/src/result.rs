use error::Error;

/// Define the result type used by the application
pub type Result<T> = ::std::result::Result<T, Error>;
