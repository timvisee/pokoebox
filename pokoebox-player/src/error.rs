use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Main application error structure.
#[derive(Debug)]
pub struct Error<'a> {
    description: &'a str
}

impl<'a> Error<'a> {

    /// Create a new application error instance.
    /// A brief description of the error must be passed to the `description` parameter.
    pub fn new(description: &'a str) -> Self {
        Error {
            description: description
        }
    }
}

/// Implement the `Error` trait, to define this structure as error.
impl<'a> error::Error for Error<'a> {

    /// Get the error description.
    fn description(&self) -> &str {
        self.description
    }
}

/// Implement the `Display` trait, required by the `Error` trait.
impl<'a> Display for Error<'a> {

    /// Format the error, to make it displayable in the console.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}