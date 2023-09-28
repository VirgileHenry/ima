/// Created by Virgile HENRY, 2023/09/28


pub mod address_modes;
pub mod control_flow;
pub mod data_type;
pub mod error;
pub mod instructions;
pub mod options;
pub mod zones;

use std::{
    time::Instant,
    io::{
        BufRead,
        Write
    }
};

use crate::{
    instructions::Instructions,
    ImaRunMode,
};

use self::{
    zones::{
        program::{Program, ReleaseModeProgram, DebugModeProgram, RunMode},
        memory::{Memory, StackPointer, Pointer},
        registers::Registers, flags::Flags,
    },
    error::ImaError,
    options::ImaOptions,
    control_flow::ImaControlFlow, address_modes::RegisterIndex,
};

#[cfg(not(feature = "public-ima"))]
pub struct IMA<RM: RunMode> {
    registers: Registers,
    code: Program<RM>,
    memory: Memory,
    flags: Flags,
    gb: StackPointer,
    lb: StackPointer,
    sp: StackPointer,
    ima_start_time: Instant,
    run_mode: ImaRunMode,
    control_flow: ImaControlFlow,
}

#[cfg(feature = "public-ima")]
pub struct IMA<RM: RunMode> {
    pub registers: Registers,
    pub code: Program<RM>,
    pub memory: Memory,
    pub flags: Flags,
    pub gb: StackPointer,
    pub lb: StackPointer,
    pub sp: StackPointer,
    pub ima_start_time: Instant,
    pub run_mode: ImaRunMode,
    pub control_flow: ImaControlFlow,
}

impl<RM: RunMode> IMA<RM> {
    /// Creates a new IMA with the given program, options, input and output.
    pub fn new(
        program: Program<RM>,
        options: ImaOptions,
    ) -> IMA<RM> {
        IMA {
            registers: Registers::new(16), // originally not changeable, but easily expandable.
            code: program,
            memory: Memory::new(options.heap_size, options.stack_size),
            flags: Flags::new(),
            gb: StackPointer::zero(),
            lb: StackPointer::zero(),
            sp: StackPointer::zero(),
            ima_start_time: Instant::now(),
            run_mode: options.run_mode,
            control_flow: ImaControlFlow::Continue,
        }
    }
}

impl IMA<ReleaseModeProgram> {
    /// Run the ima in release mode.
    pub fn run<R: BufRead, W: Write>(&mut self, input: &mut R, output: &mut W) -> Result<(), ImaError> {
        loop {
            let instruction = match self.code.fetch() {
                Some(ins) => ins.clone(),
                None => return Err(ImaError::NoMoreInstructions),
            };
            
            self.code.increment_pc();

            self.execute(instruction.clone(), input, output).map_err(|e|
                ImaError::ExecutionError{
                    error: e,
                    line: self.code.pc(),
                    instruction
                }
            )?;
            
            match self.control_flow {
                ImaControlFlow::Continue => (),
                ImaControlFlow::Halt => break Ok(()),
                ImaControlFlow::Error => break Ok(()), // todo return with error or failure ?
            }
        }
    }
}


impl IMA<DebugModeProgram> {
    /// Runs the IMA in debug mode, expecting command line arguments from the user.
    pub fn run_debug<R: BufRead, W: Write>(&mut self, input: &mut R, output: &mut W) -> Result<(), ImaError> {
        loop {
            // fetch user input
            let mut command = String::new();
            
            let (c, args) = loop {
                input.read_line(&mut command).map_err(|e| ImaError::DebugIoError(e))?;
                let command = command.trim();
                if command.is_empty() {
                    continue;
                }
                let (c, args) = command.split_at(1);

                break (c, args);
            };
            
            match (c, args) {
                ("d", "") => {
                    self.reset();
                    if let Err(e) = self.run_until_breakpoint(input, output) {
                        output.write(format!("Error: {:?}", e).as_bytes()).map_err(|e| ImaError::DebugIoError(e))?;
                    }
                },
                ("c", "") => if let Err(e) = self.run_until_breakpoint(input, output) {
                    output.write(format!("Error: {:?}", e).as_bytes()).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("a", line) => {
                    let line = line.trim();
                    match line.parse::<u32>() {
                        Ok(line) => {
                            self.code.set_breakpoint(line);
                            output.write(format!("Breakpoint set at line {}\n", line).as_bytes()).map_err(|e| ImaError::DebugIoError(e))?;
                        },
                        Err(e) => {
                            output.write(format!("Failed to parse as u32: {}", e).as_bytes()).map_err(|e| ImaError::DebugIoError(e))?;
                        }
                    }
                }
                ("e", line) => {
                    let line = line.trim();
                    match line.parse::<u32>() {
                        Ok(line) => {
                            self.code.remove_breakpoint(line);
                            writeln!(output, "Breakpoint removed at line {}", line).map_err(|e| ImaError::DebugIoError(e))?;
                        },
                        Err(e) => {
                            writeln!(output, "Failed to parse as u32: {}", e).map_err(|e| ImaError::DebugIoError(e))?;
                        }
                    }
                }
                ("s", "") => {
                    self.reset();
                    self.code.display_inst(output).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("x", "") => {
                    let instruction = match self.code.fetch() {
                        Some(ins) => ins.clone(),
                        None => return Err(ImaError::NoMoreInstructions),
                    };
                    self.code.increment_pc();
                    self.execute(instruction.clone(), input, output).map_err(|e|
                        ImaError::ExecutionError{
                            error: e,
                            line: self.code.pc(),
                            instruction
                        }
                    )?;
                    self.code.display_inst(output).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("i", "") => {
                    self.code.display_inst(output).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("p", "") => {
                    self.code.display_program(output, 1).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("l", arg) => {
                    let arg = arg.trim();
                    match arg.parse::<u32>() {
                        Ok(arg) => {
                            self.code.display_program(output, arg).map_err(|e| ImaError::DebugIoError(e))?;
                        },
                        Err(e) => {
                            writeln!(output, "Failed to parse as u32: {}", e).map_err(|e| ImaError::DebugIoError(e))?;
                        }
                    }
                }
                ("r", "") => {
                    writeln!(output, "SP  : {}", self.sp).map_err(|e| ImaError::DebugIoError(e))?;
                    writeln!(output, "GB  : {}", self.gb).map_err(|e| ImaError::DebugIoError(e))?;
                    writeln!(output, "LB  : {}", self.lb).map_err(|e| ImaError::DebugIoError(e))?;
                    self.registers.display(output).map_err(|e| ImaError::DebugIoError(e))?;
                    self.flags.display(output).map_err(|e| ImaError::DebugIoError(e))?;
                }
                ("m", args) => {
                    let split = args.trim().split(' ').collect::<Vec<&str>>();
                    let (arg1, arg2) = match split.as_slice() {
                        [arg1, arg2] => (*arg1, *arg2),
                        _ => {
                            writeln!(output, "Invalid argument number").map_err(|e| ImaError::DebugIoError(e))?;
                            continue;
                        }
                    };
                    match (arg1.trim().parse::<u32>(), arg2.trim().parse::<u32>()) {
                        (Ok(start), Ok(end)) => {
                            self.memory.display_stack(start, end, output, self.sp).map_err(|e| ImaError::DebugIoError(e))?;
                        },
                        _ => {
                            writeln!(output, "Failed to parse as u32: \"{}\" or \"{}\"", arg1, arg2).map_err(|e| ImaError::DebugIoError(e))?;
                        }
                    }
                }
                ("b", arg) => {
                    let register = match arg.trim().parse::<u8>() {
                        Ok(register) => register,
                        Err(e) => {
                            writeln!(output, "Failed to parse as u8: {}", e).map_err(|e| ImaError::DebugIoError(e))?;
                            continue;
                        }
                    };
                    match self.registers.get(RegisterIndex(register)) {
                        data_type::DataType::MemAddr(Pointer::Heap(ptr)) => {self.memory.display_block(ptr, output, register).map_err(|e| ImaError::DebugIoError(e))?;},
                        _ => {writeln!(output, "Register {} is not a memory address", register).map_err(|e| ImaError::DebugIoError(e))?;},
                    }
                }

                
                ("q", "") => break Ok(()),
                _ => {output.write(format!("Unknown Command").as_bytes()).map_err(|e| ImaError::DebugIoError(e))?;},
            }
        }
    }

    /// Run the program until a breakpoint is reached.
    /// If there is a breakpoint on the first instruction, it will be ignored.
    /// This allows to actually make progress when this is called reapeatedly.
    pub fn run_until_breakpoint<R: BufRead, W: Write>(&mut self, input: &mut R, output: &mut W) -> Result<(), ImaError> {
        loop {
            let instruction = match self.code.fetch() {
                Some(ins) => ins.clone(),
                None => return Err(ImaError::NoMoreInstructions),
            };
            
            self.code.increment_pc();

            self.execute(instruction.clone(), input, output).map_err(|e|
                ImaError::ExecutionError{
                    error: e,
                    line: self.code.pc(),
                    instruction
                }
            )?;
            
            match self.control_flow {
                ImaControlFlow::Continue => (),
                ImaControlFlow::Halt => break Ok(()),
                ImaControlFlow::Error => break Ok(()), // todo return with error or failure ?
            }

            if self.code.is_breakpoint() {
                break Ok(());
            }
        }
    }

    /// Reset the ima to its initial state.
    pub fn reset(&mut self) {
        self.registers = Registers::new(16);
        self.memory.clear();
        self.flags = Flags::new();
        self.gb = StackPointer::zero();
        self.lb = StackPointer::zero();
        self.sp = StackPointer::zero();
        self.ima_start_time = Instant::now();
        self.control_flow = ImaControlFlow::Continue;
        self.code.reset();
    }
}

