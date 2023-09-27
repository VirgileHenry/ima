/// Created by Virgile HENRY, 2023/09/28

use std::fmt::Display;
use super::{
    data_type::{
        DataType,
        DataTypeFlag
    },
    IMA,
    zones::{
        memory::Pointer,
        program::{
            CodeAddr,
            RunMode
        }
    },
    error::ImaExecutionError
};

/// Designes a register name Rm (R0, R1, R2, ...)
/// The holded value is less than the max register count, 16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct RegisterIndex(pub u8);

impl Display for RegisterIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{}", self.0)
    }
}

/// Designes any register: either Rm or SP, GB, LB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    R(RegisterIndex),
    SP,
    GB,
    LB,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Register::R(index) => write!(f, "{}", index),
            Register::SP => write!(f, "SP"),
            Register::GB => write!(f, "GB"),
            Register::LB => write!(f, "LB"),
        }
    }
}

/// Represent a DADR value. It points to a memory address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DADR {
    /// Points the address in the register, with an immediate offset.
    OffsetIndirect {
        register: Register,
        offset: i32
    },
    /// Points the address in the register,
    /// with an immediate offset and a register offset.
    OffsetAndDisplacedIndirect { 
        address_register: Register,
        register_offset: RegisterIndex,
        immediate_offset: i32,
    },
}

impl Display for DADR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DADR::OffsetIndirect { register, offset } => {
                write!(f, "{offset}({register})")
            },
            DADR::OffsetAndDisplacedIndirect { address_register, register_offset, immediate_offset } => {
                write!(f, "{immediate_offset}({address_register} + {register_offset})")
            },
        }
    }
}

/// A DVAL represent any value. It can be the value at a DADR, a register value, an immediate value or a label.
#[derive(Debug, Clone, PartialEq)]
pub enum DVAL {
    /// Value at the given dadr. Valid only if the address is valid.
    DADR(DADR),
    /// Value in the given register.
    Register(RegisterIndex),
    /// Immediate value
    Immediate(DataType),
    /// Label (code addr)
    Label(CodeAddr),
}

impl Display for DVAL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DVAL::DADR(dadr) => write!(f, "{}", dadr),
            DVAL::Register(index) => write!(f, "{}", index),
            DVAL::Immediate(value) => write!(f, "#{}", value),
            DVAL::Label(addr) => write!(f, "@ Code {}", addr),
        }
    }
}

/// Trait to allows to compute DADR.
/// Because DADR depends on register values, the DADR in itself means nothing.
/// It need a machine support to be computed.
pub trait GetDadr {
    fn get_dadr(&self, dadr: DADR) -> Result<Pointer, ImaExecutionError>;
}

/// Trait to allows to compute DVAL.
/// Because DVAL can depends on DADR or registers, the DVAL in itself means nothing.
/// It need a machine support to be computed.
pub trait GetDval: GetDadr {
    fn get_dval(&self, dval: DVAL) -> Result<DataType, ImaExecutionError>;
}

impl<RM: RunMode> GetDadr for IMA<RM> {
    fn get_dadr(&self, dadr: DADR) -> Result<Pointer, ImaExecutionError> {
        match dadr {
            DADR::OffsetIndirect { register, offset } => {
                let register_value = match register {
                    Register::GB => Pointer::Stack(self.gb),
                    Register::LB => Pointer::Stack(self.lb),
                    Register::SP => Pointer::Stack(self.sp),
                    Register::R(index) => match self.registers.get(index) {
                        DataType::MemAddr(value) => value,
                        other => return Err(ImaExecutionError::InvalidDataType {
                            expected: DataTypeFlag::MemAddr,
                            found: other.into()
                        }),
                    },
                };
                register_value.offset(offset)
            },
            DADR::OffsetAndDisplacedIndirect { address_register, register_offset, immediate_offset } => {
                let address_register_value = match address_register {
                    Register::GB => Pointer::Stack(self.gb),
                    Register::LB => Pointer::Stack(self.lb),
                    Register::SP => Pointer::Stack(self.sp),
                    Register::R(index) => match self.registers.get(index) {
                        DataType::MemAddr(value) => value,
                        other => return Err(ImaExecutionError::InvalidDataType {
                            expected: DataTypeFlag::MemAddr,
                            found: other.into()
                        }),
                    },
                };
                let register_offset_value = match self.registers.get(register_offset) {
                    DataType::Int(value) => value,
                    other => return Err(ImaExecutionError::InvalidDataType {
                        expected: DataTypeFlag::Int,
                        found: other.into()
                    }),
                };
                address_register_value.offset(register_offset_value + immediate_offset)
            },
        }
    }
}

impl<RM: RunMode> GetDval for IMA<RM> {
    fn get_dval(&self, dval: DVAL) -> Result<DataType, ImaExecutionError> {
        match dval {
            DVAL::DADR(dadr) => {
                let ptr = self.get_dadr(dadr)?;
                self.memory.get(ptr).ok_or(ImaExecutionError::InvalidMemoryAddress(ptr))
            },
            DVAL::Register(index) => Ok(self.registers.get(index)),
            DVAL::Immediate(value) => Ok(value),
            DVAL::Label(addr) => Ok(DataType::CodeAddr(addr)),
        }
    }
}