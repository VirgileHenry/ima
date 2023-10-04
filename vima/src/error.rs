use std::{fmt::Display, error::Error};


#[derive(Debug)]
pub enum VimaError {
    OptionParsing(ima_core::OptionParsingError),
    IO(std::io::Error),
    ImaParser(ima_core::ParserError),
    ImaExecution(ima_core::complete::ImaExecutionError)
}

impl From<ima_core::OptionParsingError> for VimaError {
    fn from(e: ima_core::OptionParsingError) -> Self {
        VimaError::OptionParsing(e)
    }
}

impl From<std::io::Error> for VimaError {
    fn from(e: std::io::Error) -> Self {
        VimaError::IO(e)
    }
}

impl From<ima_core::ParserError> for VimaError {
    fn from(e: ima_core::ParserError) -> Self {
        VimaError::ImaParser(e)
    }
}

impl From<ima_core::complete::ImaExecutionError> for VimaError {
    fn from(e: ima_core::complete::ImaExecutionError) -> Self {
        VimaError::ImaExecution(e)
    }
}

impl Display for VimaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VimaError::OptionParsing(e) => write!(f, "{}", e),
            VimaError::IO(e) => write!(f, "{}", e),
            VimaError::ImaParser(e) => write!(f, "{}", e),
            VimaError::ImaExecution(e) => write!(f, "{}", e),
        }
    }
}

impl Error for VimaError {}