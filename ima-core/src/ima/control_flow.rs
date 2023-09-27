/// Created by Virgile HENRY, 2023/09/28

/// Control flow for the IMA interpreter.
#[derive(Debug, Clone, Copy)]
pub enum ImaControlFlow {
    /// The machine should continue.
    Continue,
    /// The machine have been instructed to halt.
    Halt,
    /// The machine have been instructed to stop with an error.
    Error,
}