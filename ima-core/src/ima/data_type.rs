/// Created by Virgile HENRY, 2023/09/28

use std::fmt::Display;

use super::zones::{
    memory::Pointer,
    program::CodeAddr
};

pub type Int = i32;
pub type Float = f32;

/// The IMA stores data on 32 bits, but each word is data type tagged.
/// therefore this enum represent any data the machine can hold, and the type of the data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int(Int),
    Float(Float),
    CodeAddr(CodeAddr),
    MemAddr(Pointer),
    Undefined,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DataType::Int(value) => write!(f, "{}", value),
            DataType::Float(value) => write!(f, "{}", value),
            DataType::CodeAddr(value) => write!(f, "{}", value),
            DataType::MemAddr(value) => write!(f, "{}", value),
            DataType::Undefined => write!(f, "<Undefined>"),
        }
    }
}

/// Represent the type of a data. This is mostly usefull for debuging.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataTypeFlag {
    Int,
    Float,
    CodeAddr,
    MemAddr,
    Undefined,
}

impl From<DataType> for DataTypeFlag {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Int(_) => DataTypeFlag::Int,
            DataType::Float(_) => DataTypeFlag::Float,
            DataType::CodeAddr(_) => DataTypeFlag::CodeAddr,
            DataType::MemAddr(_) => DataTypeFlag::MemAddr,
            DataType::Undefined => DataTypeFlag::Undefined,
        }
    }
}

impl Display for DataTypeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypeFlag::Int => write!(f, "Int"),
            DataTypeFlag::Float => write!(f, "Float"),
            DataTypeFlag::CodeAddr => write!(f, "CodeAddr"),
            DataTypeFlag::MemAddr => write!(f, "MemAddr"),
            DataTypeFlag::Undefined => write!(f, "Undefined"),
        }
    }
}