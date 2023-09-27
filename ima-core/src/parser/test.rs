/// Created by Virgile HENRY, 2023/09/28


#[allow(unused_imports)]
use crate::{
    ima::{
        address_modes::{
            RegisterIndex,
            Register,
            DADR,
            DVAL
        },
        data_type::DataType,
        zones::memory::Pointer
    },
    instructions::Instruction,
    parser::label::LabelMap
};


#[test]
pub fn parse_register_index() {
    assert_eq!(Ok(RegisterIndex(0)), RegisterIndex::from_str("R0"), "Failed to parse register R0");
    assert_eq!(Ok(RegisterIndex(7)), RegisterIndex::from_str("R7"), "Failed to parse register R7");
    assert_eq!(Ok(RegisterIndex(15)), RegisterIndex::from_str("R15"), "Failed to parse register R15");
    assert!(RegisterIndex::from_str("").is_err(), "Parsed empty string as register");
    assert!(RegisterIndex::from_str("R").is_err(), "Parsed invalid register R");
    assert!(RegisterIndex::from_str("R16").is_err(), "Parsed invalid register R16");
    assert!(RegisterIndex::from_str("R-1").is_err(), "Parsed invalid register R-1");
}

#[test]
pub fn parse_register() {
    assert_eq!(Ok(Register::R(RegisterIndex(0))), Register::from_str("R0"), "Failed to parse register R0");
    assert_eq!(Ok(Register::R(RegisterIndex(7))), Register::from_str("R7"), "Failed to parse register R7");
    assert_eq!(Ok(Register::R(RegisterIndex(15))), Register::from_str("R15"), "Failed to parse register R15");
    assert_eq!(Ok(Register::SP), Register::from_str("SP"), "Failed to parse register SP");
    assert_eq!(Ok(Register::GB), Register::from_str("GB"), "Failed to parse register GB");
    assert_eq!(Ok(Register::LB), Register::from_str("LB"), "Failed to parse register LB");
    assert!(Register::from_str("").is_err(), "Parsed empty string as register");
    assert!(Register::from_str("XY").is_err(), "Parsed invalid register R");
    assert!(Register::from_str("some_register_name").is_err(), "Parsed invalid register R16");
}

#[test]
pub fn parse_dadr() {
    assert_eq!(
        Ok(DADR::OffsetIndirect { register: Register::GB, offset: 0 }),
        DADR::from_str("0(GB)"),
        "Failed to parse DADR 0(GB)"
    );
    assert_eq!(
        Ok(DADR::OffsetIndirect { register: Register::SP, offset: 12345 }),
        DADR::from_str("12345 ( SP )"),
        "Failed to parse DADR 12345(SP)"
    );
    assert_eq!(
        Ok(DADR::OffsetIndirect { register: Register::LB, offset: -2 }),
        DADR::from_str("-2(LB)"),
        "Failed to parse DADR -2(LB)"
    );
    assert_eq!(
        Ok(DADR::OffsetIndirect { register: Register::R(RegisterIndex(2)), offset: 0 }),
        DADR::from_str("0 (R2)"),
        "Failed to parse DADR 0(R2)"
    );
    assert_eq!(
        Ok(DADR::OffsetAndDisplacedIndirect { address_register: Register::GB, register_offset: RegisterIndex(1), immediate_offset: 0 }),
        DADR::from_str("0(GB, R1)"),
        "Failed to parse DADR 0(GB, R1)"
    );
    assert_eq!(
        Ok(DADR::OffsetAndDisplacedIndirect { address_register: Register::SP, register_offset: RegisterIndex(4), immediate_offset: -3 }),
        DADR::from_str("-3 (SP, R4)"),
        "Failed to parse DADR -3 (SP, R4)"
    );
    assert_eq!(
        Ok(DADR::OffsetAndDisplacedIndirect { address_register: Register::LB, register_offset: RegisterIndex(15), immediate_offset: -4 }),
        DADR::from_str("-4(LB,R15)"),
        "Failed to parse DADR -4(LB,R15)"
    );
    assert_eq!(
        Ok(DADR::OffsetAndDisplacedIndirect { address_register: Register::R(RegisterIndex(4)), register_offset: RegisterIndex(1), immediate_offset: 1234 }),
        DADR::from_str("1234(R4, R1)"),
        "Failed to parse DADR 1234(R4, R1)"
    );
}

#[test]
fn parse_immediate() {
    assert_eq!(Ok(DataType::Int(0)), DataType::from_str("#0"), "Failed to parse immediate #0");
    assert_eq!(Ok(DataType::Int(12345)), DataType::from_str("#12345"), "Failed to parse immediate #12345");
    assert_eq!(Ok(DataType::Int(-12345)), DataType::from_str("#-12345"), "Failed to parse immediate #-12345");
    assert_eq!(Ok(DataType::Float(0.0)), DataType::from_str("#0.0"), "Failed to parse immediate #0.0");
    assert_eq!(Ok(DataType::Float(3.1415)), DataType::from_str("#3.1415"), "Failed to parse immediate #0.0");
    assert_eq!(Ok(DataType::Float(12345.0)), DataType::from_str("#12345.0"), "Failed to parse immediate #12345.0");
    assert_eq!(Ok(DataType::Float(-12345.0)), DataType::from_str("#-12345.0"), "Failed to parse immediate #-12345.0");
    assert_eq!(Ok(DataType::MemAddr(Pointer::Null)), DataType::from_str("#null"), "Failed to parse immediate #null");
}

#[test]
fn parse_dval() {
    let label_map = LabelMap::new();
    assert_eq!(Ok(DVAL::Register(RegisterIndex(3))), DVAL::from_str("R3", &label_map), "Failed to parse DVAL R3");
    assert_eq!(Ok(DVAL::Register(RegisterIndex(15))), DVAL::from_str("R15", &label_map), "Failed to parse DVAL R15");
    assert_eq!(Ok(DVAL::DADR(DADR::OffsetIndirect { register: Register::GB, offset: 0 })), DVAL::from_str("0(GB)", &label_map), "Failed to parse DVAL 0(GB)");
    assert_eq!(Ok(DVAL::DADR(DADR::OffsetIndirect { register: Register::SP, offset: 12345 })), DVAL::from_str("12345 ( SP )", &label_map), "Failed to parse DVAL 12345(SP)");
    assert_eq!(Ok(DVAL::Immediate(DataType::Int(10))), DVAL::from_str("#10", &label_map), "Failed to parse DVAL #10");
    assert_eq!(Ok(DVAL::Immediate(DataType::Float(3.1415))), DVAL::from_str("#3.1415", &label_map), "Failed to parse DVAL #3.1415");
    assert_eq!(Ok(DVAL::Immediate(DataType::MemAddr(Pointer::Null))), DVAL::from_str("#null", &label_map), "Failed to parse DVAL #null");
}

#[test]
fn parse_instruction_add() {
    let label_map = LabelMap::new();
    assert_eq!(Ok(Instruction::ADD(DVAL::Register(RegisterIndex(0)), RegisterIndex(0))), Instruction::from_str("ADD R0, R0", &label_map), "Failed to parse ADD R0, R0");
    assert_eq!(Ok(Instruction::ADD(DVAL::DADR(DADR::OffsetIndirect { register: Register::SP, offset: -2 }), RegisterIndex(15))), Instruction::from_str("  ADD -2 ( SP ),   R15  ", &label_map), "Failed to parse ADD -2 ( SP ), R15");
    assert_eq!(Ok(Instruction::ADD(DVAL::Immediate(DataType::Int(12345)), RegisterIndex(7))), Instruction::from_str("ADD #12345, R7", &label_map), "Failed to parse ADD #12345, R7");
    assert_eq!(Ok(Instruction::ADD(DVAL::DADR(DADR::OffsetAndDisplacedIndirect { address_register: Register::R(RegisterIndex(2)), register_offset: RegisterIndex(14), immediate_offset: -167 }), RegisterIndex(1))), Instruction::from_str("ADD -167(R2, R14), R1", &label_map), "Failed to parse ADD -167(R2, R14), R1");
}