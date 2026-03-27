use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid Block")]
pub struct InvalidBlockError;

#[derive(Debug, Error)]
#[error("Invalid Row or Column numbers")]
pub struct InvalidNumbersError;

#[derive(Debug, Error)]
#[error("Invalid Board")]
pub struct InvalidBoardError;
