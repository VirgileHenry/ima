/// Created by Virgile HENRY, 2023/09/28

use crate::{
    instructions::Instruction,
    ima::zones::program::{
        Program,
        ReleaseModeProgram,
        DebugModeProgram
    }
};
use super::{
    label::{
        Label,
        LabelMap
    },
    error::{
        ParserErrorType,
        ParserError
    },
    token::{
        lex,
        Token
    }
};

/// Represent a line of code in the IMA.
#[derive(Debug)]
pub struct Line {
    /// A line can have any number of labels.
    pub labels: Vec<Label>,
    /// A line can have up to one instruction.
    pub instruction: Option<Instruction>,
    /// A line can have up to one comment.
    pub comment: Option<String>,
}

impl Line {
    /// Creates a new empty line.
    pub fn empty() -> Line {
        Line {
            labels: Vec::new(),
            instruction: None,
            comment: None,
        }
    }
}

impl Line {
    /// Parse a text line into a program line.
    /// This will attempt to parse labels and instructions.
    fn from_tokens(tokens: &Vec<Token>, label_map: &LabelMap) -> Result<Self, ParserErrorType> {
        
        let mut line = Line::empty();

        for token in tokens.iter() {
            match token {
                Token::Label(l) => line.labels.push(l.clone()),
                Token::Comment(c) => line.comment = Some(c.clone()),
                Token::Assembly(s) => line.instruction = Some(Instruction::from_str(s, label_map)?),
            }
        }

        Ok(line)
    }
}


/// Parse an input string to a program in release mode.
pub fn parse(input: &str) -> Result<Program<ReleaseModeProgram>, ParserError> {
    let mut result = Vec::new();
    let lines = lex(input).map_err(|_e| ParserError::LexerError)?;

    let mut label_map = LabelMap::new();
    label_map.scan_labels(&lines, false);

    for (line, tokens) in lines.into_iter().enumerate() {
        let line = Line::from_tokens(&tokens, &label_map).map_err(|e|
            ParserError::InnerParserError {
                line: line + 1, // line number starts at 1, and enumerate starts at 0
                error: e,
            }
        )?;
        if let Some(ins) = line.instruction {
            result.push(ins);
        }
    }

    let program = Program::new(result);
    Ok(program)
}

/// Parse an input string to a program in debug mode.
pub fn parse_debug(input: &str) -> Result<Program<DebugModeProgram>, ParserError> {
    let mut result = Vec::new();
    let lines = lex(input).map_err(|_e| ParserError::LexerError)?;

    let mut label_map = LabelMap::new();
    label_map.scan_labels(&lines, true);

    for (line, tokens) in lines.into_iter().enumerate() {
        let line = Line::from_tokens(&tokens, &label_map).map_err(|e|
            ParserError::InnerParserError {
                line: line + 1, // line number starts at 1, and enumerate starts at 0
                error: e,
            }
        )?;
        result.push(line)
    }

    let program = Program::new_debug(result);
    Ok(program)
}