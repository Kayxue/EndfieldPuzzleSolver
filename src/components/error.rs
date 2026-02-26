use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct InvalidBlockError {
    message: &'static str,
}

impl InvalidBlockError {
    pub fn new() -> InvalidBlockError {
        InvalidBlockError {
            message: "Invalid Block",
        }
    }
}

impl Display for InvalidBlockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidBlockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn description(&self) -> &str {
        self.message
    }
}
