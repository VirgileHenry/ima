/// Created by Virgile HENRY, 2023/09/28

use std::{fmt::Display, error::Error};

use super::{
    dadr::DadrParseError,
    dval::DvalParseError,
    register::RegIndexParseError
};

#[derive(Debug)]
pub enum ParserError {
    InnerParserError {
        line: usize,
        error: ParserErrorType,
    },
    LexerError,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Parser Error]: ")?;
        match self {
            ParserError::InnerParserError { line, error } => write!(f, "On line {}: {}", line, error),
            ParserError::LexerError => write!(f, "Unable to create tokens."),
        }
    }
}

/// Error that can be thrown by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorType {
    /// The given string can't be parsed as a label.
    InvalidLabel(String),
    /// The given opcode and arguments can't be parsed as an instruction.
    InvalidInstruction(String),
    /// The given string can't be parsed as a DADR.
    DadrParseError(DadrParseError),
    /// The given string can't be parsed as a DVAL.
    DvalParseError(DvalParseError),
    /// The given string can't be parsed as a register index.
    RegIndexParseError(RegIndexParseError),
    /// The given string can't be parsed as an integer.
    IntParseError(String),
}

impl From<DadrParseError> for ParserErrorType {
    fn from(e: DadrParseError) -> Self {
        ParserErrorType::DadrParseError(e)
    }
}

impl From<DvalParseError> for ParserErrorType {
    fn from(e: DvalParseError) -> Self {
        ParserErrorType::DvalParseError(e)
    }
}

impl From<RegIndexParseError> for ParserErrorType {
    fn from(e: RegIndexParseError) -> Self {
        ParserErrorType::RegIndexParseError(e)
    }
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorType::InvalidLabel(label) => write!(f, "Invalid label: {}", label),
            ParserErrorType::InvalidInstruction(instruction) => write!(f, "Invalid instruction: {}", instruction),
            ParserErrorType::DadrParseError(e) => write!(f, "{}", e),
            ParserErrorType::DvalParseError(e) => write!(f, "{}", e),
            ParserErrorType::RegIndexParseError(e) => write!(f, "{}", e),
            ParserErrorType::IntParseError(e) => write!(f, "Invalid integer: {}", e),
        }
    }
}

impl Error for ParserErrorType {}