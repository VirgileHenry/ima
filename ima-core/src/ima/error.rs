/// Created by Virgile HENRY, 2023/09/28


use std::{fmt::Display, error::Error};

use crate::instructions::Instruction;

use super::{data_type::DataTypeFlag, zones::memory::Pointer};

/// Operation error: The machine have been instructed to perform an operation,
/// but the found data types are not valid for this operation.
#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    Add(DataTypeFlag, DataTypeFlag),
    Cmp(DataTypeFlag, DataTypeFlag),
    Div(DataTypeFlag, DataTypeFlag),
    Mul(DataTypeFlag, DataTypeFlag),
    Quo(DataTypeFlag, DataTypeFlag),
    Rem(DataTypeFlag, DataTypeFlag),
    Sub(DataTypeFlag, DataTypeFlag),
    Fma(DataTypeFlag, DataTypeFlag, DataTypeFlag),
    Opp(DataTypeFlag),
    Shl(DataTypeFlag),
    Shr(DataTypeFlag),
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Add(d1, d2) => write!(f, "Add with datatypes {} and {}", d1, d2),
            OperationType::Cmp(d1, d2) => write!(f, "Compare with datatypes {} and {}", d1, d2),
            OperationType::Div(d1, d2) => write!(f, "Divide with datatypes {} and {}", d1, d2),
            OperationType::Mul(d1, d2) => write!(f, "Multiply with datatypes {} and {}", d1, d2),
            OperationType::Quo(d1, d2) => write!(f, "Quotient with datatypes {} and {}", d1, d2),
            OperationType::Rem(d1, d2) => write!(f, "Remainder with datatypes {} and {}", d1, d2),
            OperationType::Sub(d1, d2) => write!(f, "Substract with datatypes {} and {}", d1, d2),
            OperationType::Fma(d1, d2, d3) => write!(f, "Fused multiply-add with datatypes {}, {} and {}", d1, d2, d3),
            OperationType::Opp(d) => write!(f, "Opposite with datatype {}", d),
            OperationType::Shl(d) => write!(f, "Shift left with datatype {}", d),
            OperationType::Shr(d) => write!(f, "Shift right with datatype {}", d),
        }
    }
}

/// Any error the IMA machine can throw at runtime.
#[derive(Debug)]
pub enum ImaExecutionError {
    /// The stack of the machine overflowed.
    /// Values have been pushed on the stack until there is no space left.
    StackOverflow,
    /// The stack of the machine underflowed.
    /// Values have been popped from the stack until there is no value left.
    StackUnderflow,
    /// The heap of the machine overflowed.
    /// The machine tried to access a memory address that is not in the heap.
    HeapOverflow,
    /// The machine was expecting a value of a certain type, but found another type.
    InvalidDataType {
        expected: DataTypeFlag,
        found: DataTypeFlag,
    },
    /// The machine tried to access a memory address that is null.
    InvalidMemoryAddress(Pointer),
    /// The machine tried to perform an operation with invalid data types.
    InvalidOperation(OperationType),
    /// The Machine failed to read user input. This will be caused by an IO error, not a user error.
    FailedToReadInput(std::io::Error),
    /// The Machine failed to write user output. This will be caused by an IO error, not a user error.
    FailedToWriteIO(std::io::Error),
}

impl Display for ImaExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ImaExecutionError::StackOverflow => write!(f, "Stack overflow"),
            ImaExecutionError::StackUnderflow => write!(f, "Stack underflow"),
            ImaExecutionError::HeapOverflow => write!(f, "Heap overflow"),
            ImaExecutionError::InvalidDataType{expected, found} => write!(f, "Invalid data type: expected {:?}, found {:?}", expected, found),
            ImaExecutionError::InvalidMemoryAddress(ptr) => write!(f, "Invalid memory address ({ptr})"),
            ImaExecutionError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            ImaExecutionError::FailedToReadInput(e) => write!(f, "Failed to read input: {}", e),
            ImaExecutionError::FailedToWriteIO(e) => write!(f, "Failed to write output: {}", e),
        }
    }
}

impl Error for ImaExecutionError {}

#[derive(Debug)]
pub enum ImaError {
    /// An error occured during the execution of the machine.
    ExecutionError {
        error: ImaExecutionError,
        line: u32,
        instruction: Instruction,
    },
    /// The machine have no more instructions to execute.
    NoMoreInstructions,
    /// The machine failed an io operation in debug mode.
    DebugIoError(std::io::Error),
}

impl Display for ImaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Ima Error]: ")?;
        match self {
            ImaError::ExecutionError{error, line, instruction} => write!(f, "{}, at line {}: {}", error, line, instruction),
            ImaError::NoMoreInstructions => write!(f, "No more instructions"),
            ImaError::DebugIoError(e) => write!(f, "Error on debug I/O: {}. This is not a machine error, but should be due to the environment", e),
        }
    }
}

impl Error for ImaError {}