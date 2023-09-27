/// Created by Virgile HENRY, 2023/09/28

use std::fmt::Display;

use crate::ima::{
    address_modes::{
        DVAL,
        RegisterIndex,
        DADR
    },
    data_type::DataType
};

use super::label::{LabelMap, Label};

/// Error that can be thrown when parsing a DVAL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DvalParseError {
    from: String,
}

impl Display for DvalParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dval parse error: {}", self.from)
    }
}

impl DVAL {
    /// Parse a string into a DVAL.
    pub(super) fn from_str(s: &str, label_map: &LabelMap) -> Result<Self, DvalParseError> {
        let s = s.trim();
        // try all possibilities

        let _ = match RegisterIndex::from_str(s) {
            Ok(reg) => return Ok(DVAL::Register(reg)),
            Err(e) => e,
        };
        let _ = match DADR::from_str(s) {
            Ok(dadr) => return Ok(DVAL::DADR(dadr)),
            Err(e) => e,
        };
        let _ = match DataType::from_str(s) {
            Ok(data_type) => return Ok(DVAL::Immediate(data_type)),
            Err(e) => e,
        };
        let _ = match Label::from_str(s) {
            Ok(label) => {
                if let Some(address) = label_map.get(&label) {
                    return Ok(DVAL::Label(address));
                }
                else {
                    "Label not in label map".to_string()
                }
            },
            Err(_) => "Unable to parse label".to_string(),
        };

        Err(DvalParseError {
            from: s.to_string(),
        })
    }
}