/// Created by Virgile HENRY, 2023/09/28


pub use ima::{
    IMA,
    options::{
        ImaOptions,
        ImaRunMode,
        OptionParsingError,
    },
    error::ImaError,
    zones::program::{
        ReleaseModeProgram,
        DebugModeProgram,
    }
};
pub use parser::{
    error::ParserError,
    parser::{
        parse,
        parse_debug,
    },
};

mod ima;
mod parser;
mod instructions;

/// export all the types for further use.
pub mod complete {
    pub use crate::{
        parser::parser::Line,
        instructions::{
            Instruction,
            Instructions,
        },
        ima::{
            IMA,
            control_flow::ImaControlFlow,
            options::{
                ImaOptions,
                ImaRunMode,
                OptionParsingError,
            },
            data_type::DataType,
            error::{
                ImaError,
                ImaExecutionError
            },
            zones::{
                program::{
                    ReleaseModeProgram,
                    DebugModeProgram,
                    CodeAddr,
                    RunMode,
                    Program,
                },
                memory::{
                    Memory,
                    Pointer,
                    StackPointer,
                    HeapPointer,
                },
                registers::Registers,
                flags::Flags,
            },
            address_modes::{
                DVAL,
                DADR,
                RegisterIndex,
            }
        },

    };
}

#[cfg(test)]
mod test;
