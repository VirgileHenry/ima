/// Created by Virgile HENRY, 2023/09/28

use hexf_parse::parse_hexf32;

use crate::ima::{data_type::{DataType, Int, Float}, zones::memory::Pointer};




impl DataType {
    /// tries to parse an immediate as a data type.
    pub(super) fn from_str(s: &str) -> Result<Self, ()> {
        if s.len() < 2 {
            return Err(()); // empty string is not a valid data type
        }

        let (t, v) = s.split_at(1);
        match t {
            "#" => {
                // int regex: can have +/-, then at least one digit
                let int_rx = regex::Regex::new(r"^(\+|\-)?\d+$").unwrap();
                // float regex: can have +/-, then at least one digit, then a dot, then at least one digit
                let float_rx = regex::Regex::new(r"^(\+|\-)?\d+\.\d+$").unwrap();
                // null regex: can only be "null"
                let null_rx = regex::RegexBuilder::new(r"^null$").case_insensitive(true).build().unwrap();

                if int_rx.is_match(v) {
                    Ok(DataType::Int(v.parse::<Int>().unwrap()))
                } else if float_rx.is_match(v) {
                    Ok(DataType::Float(v.parse::<Float>().unwrap()))
                } else if let Ok(f) = parse_hexf32(v, false) {
                    Ok(DataType::Float(f))
                } else if null_rx.is_match(v) {
                    Ok(DataType::MemAddr(Pointer::Null))
                } else {
                    Err(())
                }
            },
            _ => Err(()),
        }
    }
}