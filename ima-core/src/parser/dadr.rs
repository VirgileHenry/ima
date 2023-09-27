/// Created by Virgile HENRY, 2023/09/28


use std::fmt::Display;

use regex::Regex;

use crate::ima::address_modes::{DADR, Register, RegisterIndex};

use super::register::{RegisterParseError, RegIndexParseError};


/// Errors for parsing DADR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DadrParseError {
    /// Error parsing a register
    RegisterParseError(RegisterParseError),
    /// Error parsing a offset (int)
    OffsetParseError(String),
    /// Error parsing a register index
    RegOffsetParseEror(RegIndexParseError),
    /// No regex matched: the string is not in the correct format
    NoRegexMatch(String),
}

impl From<RegisterParseError> for DadrParseError {
    fn from(e: RegisterParseError) -> Self {
        DadrParseError::RegisterParseError(e)
    }
}

impl From<RegIndexParseError> for DadrParseError {
    fn from(e: RegIndexParseError) -> Self {
        DadrParseError::RegOffsetParseEror(e)
    }
}

impl Display for DadrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DadrParseError::RegisterParseError(e) => write!(f, "{}", e),
            DadrParseError::OffsetParseError(e) => write!(f, "Invalid offset: {}", e),
            DadrParseError::RegOffsetParseEror(e) => write!(f, "{}", e),
            DadrParseError::NoRegexMatch(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl DADR {
    /// Parse a string to a DADR
    pub(super) fn from_str(s: &str) -> Result<Self, DadrParseError> {
        let s = s.trim();
        // todo : lazy static regex to avoid creating them for each call

        // regex: () are capture groups, ?<name> is the name of the capture group, the pattern after this is the matched group
        // (?<d>[0-9\-]+) matches a number with optional minus sign, and assign it to capture group 'd'
        // (?<reg>[a-zA-Z]+) matches a string of letters, and assign it to capture group 'reg'
        // [ ]* matches any number of spaces, to discard them
        // \( \) are escaped parenthesis
        // see https://docs.rs/regex/latest/regex/#grouping-and-flags

        // this will capture d(reg)
        let off_ind_reg = Regex::new(r"(?<d>[0-9\-]+)[ ]*\([ ]*(?<reg>[a-zA-Z0-9]+)[ ]*\)").unwrap();
        // this will capture d(reg, dis)
        let off_dis_ind_reg = Regex::new(r"(?<d>[0-9\-]+)[ ]*\([ ]*(?<reg>[a-zA-Z0-9]+),[ ]*(?<dis>[a-zA-Z0-9]+)[ ]*\)").unwrap();

        if let Some(caps) = off_ind_reg.captures(s) {
            let offset = caps["d"].parse::<i32>().or(Err(DadrParseError::OffsetParseError(caps["d"].to_string())))?;
            let register = Register::from_str(&caps["reg"])?;
            Ok(DADR::OffsetIndirect { register, offset })
        } else if let Some(caps) = off_dis_ind_reg.captures(s) {
            let immediate_offset = caps["d"].parse::<i32>().or(Err(DadrParseError::OffsetParseError(caps["d"].to_string())))?;
            let address_register = Register::from_str(&caps["reg"])?;
            let register_offset = RegisterIndex::from_str(&caps["dis"])?;
            Ok(DADR::OffsetAndDisplacedIndirect { address_register, register_offset, immediate_offset })
        } else {
            Err(DadrParseError::NoRegexMatch(s.to_string()))
        }
    }
}