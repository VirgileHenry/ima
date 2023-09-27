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

#[cfg(test)]
mod test;
