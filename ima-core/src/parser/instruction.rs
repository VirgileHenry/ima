/// Created by Virgile HENRY, 2023/09/28

use crate::{
    ima::address_modes::{
        DVAL,
        RegisterIndex,
        DADR
    },
    instructions::Instruction
};

use super::{label::LabelMap, error::ParserErrorType};


impl Instruction {
    /// Parse a string into an instruction.
    pub(super) fn from_str(s: &str, label_map: &LabelMap) -> Result<Self, ParserErrorType> {
        let s = s.trim_start(); // let's avoid finding a space before instruction
        // split at first space to get opcode
        let (instr, args) = match s.find(' ') {
            Some(index) => (
                // get the arguments
                s[..index].trim(),
                split_args(&s[index+1..].trim())
            ),
            None => (s, vec![]),
        };
        // match on opcode and argument number, then for each argument, try to parse it to the correct type
        match (instr, args.as_slice()) {
            ("ADD", [dval, rm]) => Ok(Instruction::ADD(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("ADDSP", [v]) => Ok(Instruction::ADDSP(try_parse_uint(v)?)),
            ("BEQ", [dval]) => Ok(Instruction::BEQ(DVAL::from_str(dval, label_map)?)),
            ("BGE", [dval]) => Ok(Instruction::BGE(DVAL::from_str(dval, label_map)?)),
            ("BGT", [dval]) => Ok(Instruction::BGT(DVAL::from_str(dval, label_map)?)),
            ("BLE", [dval]) => Ok(Instruction::BLE(DVAL::from_str(dval, label_map)?)),
            ("BLT", [dval]) => Ok(Instruction::BLT(DVAL::from_str(dval, label_map)?)),
            ("BNE", [dval]) => Ok(Instruction::BNE(DVAL::from_str(dval, label_map)?)),
            ("BOV", [dval]) => Ok(Instruction::BOV(DVAL::from_str(dval, label_map)?)),
            ("BRA", [dval]) => Ok(Instruction::BRA(DVAL::from_str(dval, label_map)?)),
            ("BSR", [dval]) => Ok(Instruction::BSR(DVAL::from_str(dval, label_map)?)),
            ("CLK", []) => Ok(Instruction::CLK),
            ("CMP", [dval, rm]) => Ok(Instruction::CMP(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("DEL", [rm]) => Ok(Instruction::DEL(RegisterIndex::from_str(rm)?)),
            ("DIV", [dval, rm]) => Ok(Instruction::DIV(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("ERROR", []) => Ok(Instruction::ERROR),
            ("FLOAT", [dval, rm]) => Ok(Instruction::FLOAT(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("FMA", [dval, rm]) => Ok(Instruction::FMA(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("HALT", []) => Ok(Instruction::HALT),
            ("INT", [dval, rm]) => Ok(Instruction::INT(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("LEA", [dadr, rm]) => Ok(Instruction::LEA(DADR::from_str(dadr)?, RegisterIndex::from_str(rm)?)),
            ("LOAD", [dval, rm]) => Ok(Instruction::LOAD(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("MUL", [dval, rm]) => Ok(Instruction::MUL(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("NEW", [dval, rm]) => Ok(Instruction::NEW(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("OPP", [dval, rm]) => Ok(Instruction::OPP(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("PEA", [dadr]) => Ok(Instruction::PEA(DADR::from_str(dadr)?)),
            ("POP", [rm]) => Ok(Instruction::POP(RegisterIndex::from_str(rm)?)),
            ("PUSH", [rm]) => Ok(Instruction::PUSH(RegisterIndex::from_str(rm)?)),
            ("QUO", [dval, rm]) => Ok(Instruction::QUO(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("REM", [dval, rm]) => Ok(Instruction::REM(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("RFLOAT", []) => Ok(Instruction::RFLOAT),
            ("RINT", []) => Ok(Instruction::RINT),
            ("RTS", []) => Ok(Instruction::RTS),
            ("RUTF8", []) => Ok(Instruction::RUTF8),
            ("SCLK", []) => Ok(Instruction::SCLK),
            ("SEQ", [rm]) => Ok(Instruction::SEQ(RegisterIndex::from_str(rm)?)),
            ("SETROUND_DOWNWARD", []) => Ok(Instruction::SETROUND_DOWNWARD),
            ("SETROUND_TONEAREST", []) => Ok(Instruction::SETROUND_TONEAREST),
            ("SETROUND_TOWARDZERO", []) => Ok(Instruction::SETROUND_TOWARDZERO),
            ("SETROUND_UPWARD", []) => Ok(Instruction::SETROUND_UPWARD),
            ("SGE", [rm]) => Ok(Instruction::SGE(RegisterIndex::from_str(rm)?)),
            ("SGT", [rm]) => Ok(Instruction::SGT(RegisterIndex::from_str(rm)?)),
            ("SHL", [rm]) => Ok(Instruction::SHL(RegisterIndex::from_str(rm)?)),
            ("SHR", [rm]) => Ok(Instruction::SHR(RegisterIndex::from_str(rm)?)),
            ("SLE", [rm]) => Ok(Instruction::SLE(RegisterIndex::from_str(rm)?)),
            ("SLT", [rm]) => Ok(Instruction::SLT(RegisterIndex::from_str(rm)?)),
            ("SNE", [rm]) => Ok(Instruction::SNE(RegisterIndex::from_str(rm)?)),
            ("SOV", [rm]) => Ok(Instruction::SOV(RegisterIndex::from_str(rm)?)),
            ("STORE", [rm, dadr]) => Ok(Instruction::STORE(RegisterIndex::from_str(rm)?, DADR::from_str(dadr)?)),
            ("SUB", [dval, rm]) => Ok(Instruction::SUB(DVAL::from_str(dval, label_map)?, RegisterIndex::from_str(rm)?)),
            ("SUBSP", [v]) => Ok(Instruction::SUBSP(try_parse_uint(v)?)),
            ("TSTO", [v]) => Ok(Instruction::TSTO(try_parse_uint(v)?)),
            ("WFLOAT", []) => Ok(Instruction::WFLOAT),
            ("WFLOATX", []) => Ok(Instruction::WFLOATX),
            ("WNL", []) => Ok(Instruction::WNL),
            ("WINT", []) => Ok(Instruction::WINT),
            ("WSTR", [s]) => Ok(Instruction::WSTR(s[1..s.len()-1].replace("\"\"", "\""))),
            ("WUTF8", []) => Ok(Instruction::WUTF8),

            _ => Err(ParserErrorType::InvalidInstruction(s.to_string())),
        }
    }
}

/// Split the given arguments of an opcode into a vector of arguments.
/// The split is made at every comma, excepting:
/// - commas inside parenthesis like '1(R1, R2)',
/// - commas inside quotes '"Hello, world!"',
fn split_args(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    // char indices allow all char types, not just ascii. This will avoid taking slices at the mid of a char.
    let mut char_index = input.char_indices().peekable();

    let mut buffer_start = 0;
    let mut buffer_end = 0;

    while let Some((i, c)) = char_index.next() {
        match c {
            '(' => skip_until_closed_paren(&mut char_index, &mut buffer_end),
            '"' => skip_until_closed_quote(&mut char_index, &mut buffer_end),
            ',' => {
                result.push(input[buffer_start..buffer_end].trim());
                // we know the next index will be i+1, as we matched against a utf8 char
                buffer_start = i + 1;
                buffer_end = i + 1;
            }
            _ => buffer_end = i + c.len_utf8(),
        }
    }

    // add the last argument
    if buffer_start < input.len() {
        result.push(input[buffer_start..].trim());
    }

    result
}

fn skip_until_closed_paren(char_index: &mut impl Iterator<Item = (usize, char)>, buffer_end: &mut usize) {
    while let Some((i, c)) = char_index.next() {
        match c {
            ')' => {
                // here, 1 is ')'.len_utf8()
                *buffer_end = i + 1;
                return;
            }
            '(' => skip_until_closed_paren(char_index, buffer_end),
            _ => {}
        }
    }
}

fn skip_until_closed_quote(char_index: &mut std::iter::Peekable<impl Iterator<Item = (usize, char)>>, buffer_end: &mut usize) {
    while let Some((i, c)) = char_index.next() {
        match c {
            '"' => match char_index.peek() {
                Some((_, '"')) => {
                    char_index.next();
                }
                _ => {
                    *buffer_end = i;
                    return;
                }
            }
            _ => {}
        }
    }
}

fn try_parse_uint(s: &str) -> Result<u32, ParserErrorType> {
    if s.is_empty() {
        return Err(ParserErrorType::IntParseError(s.to_string()));
    }

    let (p, s) = s.split_at(1);
    match (p, s.parse::<u32>()) {
        ("#", Ok(i)) => Ok(i),
        _ => Err(ParserErrorType::IntParseError(s.to_string())),
    }
}