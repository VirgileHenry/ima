/// Created by Virgile HENRY, 2023/09/28

use std::fmt::Display;

use crate::ima::address_modes::{RegisterIndex, Register};


/// Error parsing a register index
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegIndexParseError {
    from: String,
}

impl Display for RegIndexParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid register index: {}", self.from)
    }
}

impl RegisterIndex {
    /// Parse a string to a register index
    pub(super) fn from_str(s: &str) -> Result<Self, RegIndexParseError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(RegIndexParseError{ from: s.to_string() });
        }
        let (r, n) = s.split_at(1);
        if r.to_ascii_uppercase() != "R" {
            return Err(RegIndexParseError{ from: s.to_string() });
        }
        let index = n.parse::<u8>().map_err(|_| RegIndexParseError{ from: s.to_string() })?;
        if index < 16 {
            Ok(RegisterIndex(index))
        } else {
            Err(RegIndexParseError{ from: s.to_string() })
        }
    }
}

/// Error parsing a register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterParseError {
    from: String,
}

impl From<RegIndexParseError> for RegisterParseError {
    fn from(e: RegIndexParseError) -> Self {
        RegisterParseError{ from: e.from }
    }
}

impl Display for RegisterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid register: {}", self.from)
    }
}

impl Register {
    /// Parse a string to a register
    pub(super) fn from_str(s: &str) -> Result<Self, RegisterParseError> {
        let s = s.trim();
        match s.to_ascii_uppercase().as_str() {
            "SP" => Ok(Register::SP),
            "GB" => Ok(Register::GB),
            "LB" => Ok(Register::LB),
            _ => Ok(Register::R(RegisterIndex::from_str(s)?)),
        }
    }
}