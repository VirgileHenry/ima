use crossterm::event;
use ima_core::{*, complete::ImaControlFlow};

use ratatui::{
    Terminal,
    prelude::Backend,
    Frame,
    terminal::CompletedFrame,
};

use crate::{io::IO, error::VimaError};

mod instructions;
mod ui;

/// Wrapper around a real IMA, that will intercept I/O.
pub struct VisualIMA<'a, B: Backend> {
    ima: IMA<DebugModeProgram>,
    terminal: &'a mut Terminal<B>,
    debug_io: IO,
    ima_io: IO,
    ima_io_mode: bool,
}

impl<'a, B: Backend> VisualIMA<'a, B> {

    pub fn new(ima: IMA<DebugModeProgram>, terminal: &'a mut Terminal<B>) -> Self {
        Self {
            ima,
            terminal,
            debug_io: IO::default(),
            ima_io: IO::default(),
            ima_io_mode: false,
        }
    }

    fn render(&mut self) -> std::io::Result<CompletedFrame> {
        self.terminal.draw(|frame| draw_ima(frame, &self.ima, &self.debug_io, &self.ima_io, self.ima_io_mode))
    }

    pub fn run(mut self) -> Result<(), VimaError> {

        loop {
            self.render()?;

            let event = event::read()?;
            match event {
                event::Event::Key(k) if k.kind == event::KeyEventKind::Press => match k.code {
                    event::KeyCode::Esc => return Ok(()),
                    event::KeyCode::Char(c) => self.debug_io.enter_char(c),
                    event::KeyCode::Backspace => self.debug_io.delete_char(),
                    event::KeyCode::Left => self.debug_io.move_cursor_left(),
                    event::KeyCode::Right => self.debug_io.move_cursor_right(),
                    event::KeyCode::Enter => {
                        let input = self.debug_io.input();
                        self.debug_io.flush_input();
                        self.debug_io.new_line();
                        self.execute_debug_command(&input)?;
                    },
                    _ => {},
                }
                _ => {},
            }

            // TODO : allow reset and go again.
            match self.ima.control_flow {
                ImaControlFlow::Continue => (),
                ImaControlFlow::Halt => {
                    self.debug_io.flush_input();
                    self.debug_io.new_line();
                    self.debug_io.concat_line("IMA - Halt.");
                    self.debug_io.flush_input();
                    self.debug_io.new_line();
                    self.debug_io.concat_line("Appuyez sur une touche pour quitter...");
                    break
                },
                ImaControlFlow::Error => {
                    self.debug_io.flush_input();
                    self.debug_io.new_line();
                    self.debug_io.concat_line("IMA - Error.");
                    self.debug_io.flush_input();
                    self.debug_io.new_line();
                    self.debug_io.concat_line("Appuyez sur une touche pour quitter...");
                    break
                },
            }
        }

        self.render()?;

        loop {
            let event = event::read()?;
            match event {
                event::Event::Key(k) => if k.kind == event::KeyEventKind::Press { break Ok(()) },
                _ => (),
            }
        }
    }

    fn execute_debug_command(&mut self, command: &str) -> Result<(), VimaError> {
        if command.is_empty() {
            return Ok(());
        }
        
        let (c, args) = command.split_at(1);

        match (c, args) {
            ("x", "") => self.execute_instr()?,
            ("c", "") => self.execute_until_breakpoint()?,
            ("a", arg) => {
                match arg.trim().parse::<u32>() {
                    Ok(n) => self.ima.code.set_breakpoint(n),
                    Err(_) => {
                        self.debug_io.concat_line("Invalid argument: exepected u32");
                        self.debug_io.new_line();
                    }
                }
            }
            ("e", arg) => {
                match arg.trim().parse::<u32>() {
                    Ok(n) => self.ima.code.remove_breakpoint(n),
                    Err(_) => {
                        self.debug_io.concat_line("Invalid argument: exepected u32");
                        self.debug_io.new_line();
                    }
                }
            }


            _ => {
                self.debug_io.concat_line("Unknown command.");
                self.debug_io.new_line();
            },
        }

        Ok(())
    }

    fn execute_instr(&mut self) -> Result<(), VimaError> {
        let instruction = match self.ima.code.fetch() {
            Some(ins) => ins.clone(),
            None => return Ok(()),
        };
        self.ima.code.increment_pc();

        // input is hijacked anyway, so we can just pass an empty one
        let mut input = std::io::Cursor::new(b"");
        let mut output = Vec::new();

        let result = self.execute(instruction.clone(), &mut input, &mut output);

        // read what the instruction wrote to output
        // safety: we know utf8 chars have been written anyway
        let output = String::from_utf8(output).unwrap();
        for c in output.chars() {
            match c {
                '\n' => {
                    self.ima_io.flush_input();
                    self.ima_io.new_line();
                    self.ima_io.reset_cursor();
                }
                _ => self.ima_io.enter_char(c),
            }
        }

        match result {
            Err(VimaError::ImaExecution(e)) => {
                // catch ima execution errors, as we have to display them, they are not fatal errors in debug mode.
                let result_output = format!("{e}");
                for c in result_output.chars() {
                    match c {
                        '\n' => {
                            self.ima_io.flush_input();
                            self.ima_io.new_line();
                            self.ima_io.reset_cursor();
                        }
                        _ => self.ima_io.enter_char(c),
                    }
                }
                self.ima_io.flush_input();
                self.ima_io.new_line();
                Ok(())
            }
            _ => result
        }

    }

    fn execute_until_breakpoint(&mut self) -> Result<(), VimaError> {
        loop {
            self.execute_instr()?;

            self.render()?;

            if self.ima.code.is_breakpoint() {
                break Ok(());
            }

            match self.ima.control_flow {
                ImaControlFlow::Continue => (),
                ImaControlFlow::Halt => break Ok(()),
                ImaControlFlow::Error => break Ok(()),
            }

            // safeguard: if there is a key press, stop
            match event::poll(std::time::Duration::from_nanos(0)) {
                Ok(true) => match event::read()? {
                    event::Event::Key(k) => if k.kind == event::KeyEventKind::Press { break Ok(()) },
                    _ => (),
                }
                _ => (),
            }
        }
    }
}

fn draw_ima<B: Backend>(frame: &mut Frame<B>, ima: &IMA<DebugModeProgram>, debug_io: &IO, ima_io: &IO, ima_io_mode: bool) {
    let (
        debug_area,
        ima_io_area,
        program_zone,
        stack_zone,
        heap_zone,
        register_zone,
        flag_zone,
        energy_zone,
    ) = ui::split_area(frame.size());

    ui::draw_io(frame, debug_area, debug_io, !ima_io_mode, "Enter Commands");
    ui::draw_io(frame, ima_io_area, ima_io, ima_io_mode, "IMA I/O");
    ui::draw_program(frame, program_zone, ima);
    ui::draw_registers(frame, register_zone, ima);
    ui::draw_stack(frame, stack_zone, ima);
    ui::draw_heap(frame, heap_zone, ima);
    ui::draw_energy(frame, energy_zone, ima);
    ui::draw_flags(frame, flag_zone, ima);
}
