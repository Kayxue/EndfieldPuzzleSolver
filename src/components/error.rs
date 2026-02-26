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

#[derive(Debug)]
pub struct InvalidNumbersError {
    message: &'static str,
}

impl InvalidNumbersError {
    pub fn new() -> InvalidNumbersError {
        InvalidNumbersError {
            message: "Invalid Row or Column numbers",
        }
    }
}

impl Display for InvalidNumbersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidNumbersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn description(&self) -> &str {
        self.message
    }
}

#[derive(Debug)]
pub struct InvalidBoardError {
    message: &'static str,
}

impl InvalidBoardError {
    pub fn new() -> InvalidBoardError {
        InvalidBoardError {
            message: "Invalid Board",
        }
    }
}

impl Display for InvalidBoardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidBoardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn description(&self) -> &str {
        self.message
    }
}