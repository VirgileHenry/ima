/// Created by Virgile HENRY, 2023/09/28

use std::{error::Error, fmt::Display};


#[derive(Debug, Clone)]
pub enum OptionParsingError {
    InvalidArgumentFormat{
        for_arg: String,
        found: String,
    },
    MissingArgumentValue{
        for_arg: String,
    },
    NoFileProvided,
}

impl Display for OptionParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse options: ")?;
        match self {
            OptionParsingError::InvalidArgumentFormat{for_arg, found} => write!(f, "Invalid argument format for argument {}: {}", for_arg, found),
            OptionParsingError::MissingArgumentValue{for_arg} => write!(f, "Missing argument value for argument {}", for_arg),
            OptionParsingError::NoFileProvided => write!(f, "No file provided"),
        }
    }
}

impl Error for OptionParsingError {}

/// Run mode for the IMA Machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImaRunMode {
    /// Run the ima machine.
    Run,
    /// Run the ima machine in debug mode.
    Debug,
    /// Run the ima machine, and at the end show the number of cycles spent.
    Stats,
    /// Run the ima machine, and write a newline after writing int, floats or strings.
    WriteNewLines,
}

/// IMA options.
#[derive(Debug, Clone)]
pub struct ImaOptions {
    /// Run mode for the ima machine.
    pub run_mode: ImaRunMode,
    /// Size of the stack in words.
    pub stack_size: usize,
    /// Size of the heap in words.
    pub heap_size: usize,
    /// path to file
    pub file: String,
}

impl Default for ImaOptions {
    fn default() -> ImaOptions {
        ImaOptions {
            run_mode: ImaRunMode::Run,
            stack_size: 10_000,
            heap_size: 10_000,
            file: String::new(),
        }
    }
}

impl ImaOptions {
    /// Parse the options from the command line arguments into ImaOptions.
    pub fn new(args: std::env::Args) -> Result<ImaOptions, OptionParsingError> {
        let mut options = ImaOptions::default();
        let mut args = args.skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-d" => options.run_mode = ImaRunMode::Debug,
                "-s" => options.run_mode = ImaRunMode::Stats,
                "-r" => options.run_mode = ImaRunMode::WriteNewLines,
                "-p" => {
                    let stack_size = args.next().ok_or(OptionParsingError::MissingArgumentValue {
                        for_arg: "-p".to_string(),
                    })?.parse::<usize>().map_err(|_| OptionParsingError::InvalidArgumentFormat{
                        for_arg: "-p".to_string(),
                        found: arg.to_string(),
                    })?;
                    options.stack_size = stack_size;
                }
                "-t" => {
                    let heap_size = args.next().ok_or(OptionParsingError::MissingArgumentValue{
                        for_arg: "-t".to_string(),
                    })?.parse::<usize>().map_err(|_| OptionParsingError::InvalidArgumentFormat{
                        for_arg: "-t".to_string(),
                        found: arg.to_string(),
                    })?;
                    options.heap_size = heap_size;
                }
                _ => {
                    // anything else is the file name
                    options.file = arg;
                    return Ok(options);
                },
            }
        }

        Err(OptionParsingError::NoFileProvided)
    }
}