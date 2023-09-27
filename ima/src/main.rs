/// Created by Virgile HENRY, 2023/09/28

use std::{fmt::Display, error::Error};

pub use ima_core::*;

#[derive(Debug)]
pub enum ImaInterpreterError {
    FileNotFound(std::io::Error),
    ImaError(ImaError),
    ParserError(ParserError),
    OptionParsingError(OptionParsingError),
}

impl From<ImaError> for ImaInterpreterError {
    fn from(e: ImaError) -> Self {
        ImaInterpreterError::ImaError(e)
    }
}

impl From<ParserError> for ImaInterpreterError {
    fn from(e: ParserError) -> Self {
        ImaInterpreterError::ParserError(e)
    }
}

impl From<OptionParsingError> for ImaInterpreterError {
    fn from(e: OptionParsingError) -> Self {
        ImaInterpreterError::OptionParsingError(e)
    }
}

impl Display for ImaInterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImaInterpreterError::FileNotFound(e) => write!(f, "[IO Error]: File not found ({e})"),
            ImaInterpreterError::ImaError(e) => write!(f, "{}", e),
            ImaInterpreterError::ParserError(e) => write!(f, "{}", e),
            ImaInterpreterError::OptionParsingError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ImaInterpreterError {}

fn main() {
    let res = run();

    match res {
        Ok(_) => {},
        Err(e) => {
            eprintln!("[Error] {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), ImaInterpreterError> {
    let options = ImaOptions::new(std::env::args())?;

    let file = match std::fs::read_to_string(&options.file) {
        Ok(s) => s,
        Err(e) => return Err(ImaInterpreterError::FileNotFound(e)),
    };
    
    let stdio = std::io::stdin();
    let mut input = stdio.lock();
    let mut output = std::io::stdout();
    
    match options.run_mode {
        ImaRunMode::Debug => {
            let program = parse_debug(&file)?;
            let mut ima = IMA::new(program, options);
            ima.run_debug(&mut input, &mut output)?;
        },
        _ => {
            let program = parse(&file)?;
            let mut ima = IMA::new(program, options);
            ima.run(&mut input, &mut output)?;
        },  
    };

    Ok(())
}