/// Created by Virgile HENRY, 2023/09/28

use std::{collections::HashMap, fmt::Display};

use super::token::Token;

/// A label is a string that can be used to reference a line of code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Label {
    /// Creates a new label from a string. This will fail if the string is not in the correct format.
    pub(super) fn from_str(s: &str) -> Result<Self, ()> {
        // todo: check for invalid characters / label names
        if s.is_empty() {
            Err(())
        } else {
            Ok(Label(s.to_lowercase()))
        }
    }
}

/// A line number is the number of a line in the source code.
pub type LineNumber = u32;

/// The label map maps string hard-coded labels to code addresses.
#[derive(Debug)]
pub struct LabelMap {
    labels: HashMap<Label, LineNumber>,
}

impl LabelMap {
    /// Creates a new empty label map.
    pub fn new() -> LabelMap {
        LabelMap {
            labels: HashMap::new(),
        }
    }

    /// Fill the label map with the labels found in the given lines.
    /// in debug mode, will also count empty, comment and label-only lines.
    pub fn scan_labels<'a>(&mut self, lines: &Vec<Vec<Token>>, debug_mode: bool) {
        
        let mut line_number = 0;
        for line in lines.iter() {
            let mut contains_instr = false;
            for token in line.iter() {
                match token {
                    Token::Label(label) => {self.labels.insert(label.clone(), line_number);},
                    Token::Assembly(_) => contains_instr = true,
                    _ => {},
                }
            }
            if debug_mode || contains_instr {
                line_number += 1;
            }
        }
    }

    /// Get the line number of the given label.
    pub fn get(&self, label: &Label) -> Option<LineNumber> {
        self.labels.get(label).map(|l| *l)
    }
}