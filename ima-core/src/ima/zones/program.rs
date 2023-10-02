/// Created by Virgile HENRY, 2023/09/28

use std::io::Write;

use crate::{
    instructions::Instruction,
    parser::parser::Line
};

/// Address of an instruction in the program.
pub type CodeAddr = u32;

/// Represent a program in the IMA, in release mode.
/// All lines of the program have been compacted to keep only the instructions.
pub struct ReleaseModeProgram(pub Vec<Instruction>);

/// Represent a program in the IMA, in debug mode.
/// All lines of the program are kept, with the instructions, but also comments and labels.
/// Every line also has a stop flag, which indicates breakpoints.
pub struct DebugModeProgram(pub Vec<(Line, bool)>);

/// Trait to abstract the difference between release and debug mode.
pub trait RunMode {
    /// Fetch the current instruction at the given program pointer.
    fn fetch(&self, pc: CodeAddr) -> Option<&Instruction>;
    /// Increment the given program counter.
    fn increment_pc(&mut self, pc: &mut CodeAddr);
    /// Set the given program counter to the given value.
    fn set_pc(&mut self, pc: &mut CodeAddr, new_pc: CodeAddr);
}

impl RunMode for ReleaseModeProgram {
    fn fetch(&self, pc: CodeAddr) -> Option<&Instruction> {
        self.0.get(pc as usize)
    }

    fn increment_pc(&mut self, pc: &mut CodeAddr) {
        *pc += 1;
    }

    fn set_pc(&mut self, pc: &mut CodeAddr, new_pc: CodeAddr) {
        *pc = new_pc;
    }
}

impl RunMode for DebugModeProgram {
    fn fetch(&self, pc: CodeAddr) -> Option<&Instruction> {
        match self.0.get(pc as usize) {
            Some(line) => line.0.instruction.as_ref(),
            None => None,
        }
    }

    fn increment_pc(&mut self, pc: &mut CodeAddr) {
        *pc += 1;
        while match self.0.get(*pc as usize) {
            Some(line) => line.0.instruction.is_none(),
            None => false,
        } {
            *pc += 1;
        }
    }

    fn set_pc(&mut self, pc: &mut CodeAddr, new_pc: CodeAddr) {
        *pc = new_pc;
        while match self.0.get(*pc as usize) {
            Some(line) => line.0.instruction.is_none(),
            None => false,
        } {
            *pc += 1;
        }
    }
}

#[cfg(not(feature = "public-ima"))]
/// Represent a program in the IMA.
pub struct Program<RM> {
    /// The program pointer.
    pc: CodeAddr,
    /// The code of the program.
    code: RM,
}

#[cfg(feature = "public-ima")]
/// Represent a program in the IMA.
pub struct Program<RM> {
    /// The program pointer.
    pub pc: CodeAddr,
    /// The code of the program.
    pub code: RM,
}

impl Program<ReleaseModeProgram> {
    /// Creates a new program in release mode.
    pub fn new(code: Vec<Instruction>) -> Program<ReleaseModeProgram> {
        Program { 
            pc: 0,
            code: ReleaseModeProgram(code),
        }
    }
}

impl Program<DebugModeProgram> {
    /// Creates a new program in debug mode.
    pub fn new_debug(code: Vec<Line>) -> Program<DebugModeProgram> {
        Program { 
            pc: {
                let mut pc = 0;
                while match code.get(pc as usize) {
                    Some(line) => line.instruction.is_none(),
                    None => false,
                } { pc += 1; }
                pc
            },
            code: DebugModeProgram(code.into_iter().map(|line| (line, false)).collect()),
        }
    }

    /// Toggle the breakpoint of the given line.
    pub fn toggle_breakpoint(&mut self, at: CodeAddr) {
        let mut at = at;
        loop {
            match self.code.0.get_mut(at as usize) {
                Some(line) => {
                    if line.0.instruction.is_some() {
                        line.1 = !line.1;
                        break;
                    }
                    else {
                        at += 1;
                    }
                },
                None => break,
            }
        }
    }

    /// Set the breakpoint of the given line.
    pub fn set_breakpoint(&mut self, at: CodeAddr) {
        let mut at = at;
        loop {
            match self.code.0.get_mut(at as usize) {
                Some(line) => {
                    if line.0.instruction.is_some() {
                        line.1 = true;
                        break;
                    }
                    else {
                        at += 1;
                    }
                },
                None => break,
            }
        }
    }

    /// Set the breakpoint of the given line.
    pub fn remove_breakpoint(&mut self, at: CodeAddr) {
        let mut at = at;
        loop {
            match self.code.0.get_mut(at as usize) {
                Some(line) => {
                    if line.0.instruction.is_some() {
                        line.1 = false;
                        break;
                    }
                    else {
                        at += 1;
                    }
                },
                None => break,
            }
        }
    }

    /// Check if the given line has a breakpoint.
    pub fn is_breakpoint(&mut self) -> bool {
        match self.code.0.get(self.pc as usize) {
            Some(line) => line.1,
            None => false,
        }
    }

    /// Reset the program counter to the first instruction.
    pub fn reset(&mut self) {
        self.pc = {
            let mut pc = 0;
            while match self.code.0.get(pc as usize) {
                Some(line) => line.0.instruction.is_none(),
                None => false,
            } { pc += 1; }
            pc
        };
    }

    pub fn display_inst(&self, output: &mut impl Write) -> Result<(), std::io::Error> {
        match self.code.0.get(self.pc as usize) {
            Some((Line { instruction: Some(inst), .. }, _)) => {
                output.write(format!("{}: {}\n", self.pc, inst).as_bytes())?;
                Ok(())
            },
            _ => Ok(()),
        }
    }

    pub fn display_program(&mut self, output: &mut impl Write, step: u32) -> Result<(), std::io::Error> {
        for i in 0..10 {
            let i = self.pc + i * step;
            match self.code.0.get(i as usize) {
                Some((line, bp)) => {
                    let sp = if self.pc == i { " --> " } else { "     " };
                    let bp = if *bp { "**" } else { "  " };
                    output.write(format!("{sp} {bp}{i:>4}| ").as_bytes())?;
                    for label in line.labels.iter() {
                        output.write(format!("{}: ", label).as_bytes())?;
                    }
                    match line.instruction {
                        Some(ref inst) => {output.write(format!("{}", inst).as_bytes())?;},
                        None => {},
                    }
                    match line.comment {
                        Some(ref comment) => {output.write(format!(" ; {}", comment).as_bytes())?;},
                        None => {},
                    }
                    output.write(b"\n")?;
                },
                None => break,
            }
        }
        Ok(())
    }
}



impl<RM: RunMode> Program<RM> {
    /// Get the current value of the program counter.
    pub fn pc(&self) -> CodeAddr {
        self.pc
    }

    /// Get the current instruction at the program counter.
    pub fn fetch(&self) -> Option<&Instruction> {
        self.code.fetch(self.pc)
    }

    /// Increment the program counter.
    pub fn increment_pc(&mut self) {
        self.code.increment_pc(&mut self.pc);
    }

    /// Set the program counter to the given value.
    pub fn set_pc(&mut self, pc: CodeAddr) {
        self.code.set_pc(&mut self.pc, pc);
    }

    pub fn code(&self) -> &RM {
        &self.code
    }

    pub fn code_mut(&mut self) -> &mut RM {
        &mut self.code
    }
}
