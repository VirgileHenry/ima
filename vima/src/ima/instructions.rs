use std::io::{BufRead, Write};

use ima_core::complete::*;
use crate::error::VimaError;

use super::VisualIMA;

use ratatui::prelude::Backend;

impl<'a, B: Backend> VisualIMA<'a, B> {
    pub fn execute<R: BufRead, W: Write>(&mut self, instruction: Instruction, input: &mut R, output: &mut W) -> Result<(), VimaError> {
        match instruction {
            Instruction::RINT => {
                let cycle_cost = instruction.cycle_cost(&self.ima.flags);
                self.redirected_rint(input)?;
                self.ima.cycle_count += cycle_cost;
            }
            Instruction::RFLOAT => {
                let cycle_cost = instruction.cycle_cost(&self.ima.flags);
                self.redirected_rfloat(input)?;
                self.ima.cycle_count += cycle_cost;
            }
            Instruction::RUTF8 => {
                let cycle_cost = instruction.cycle_cost(&self.ima.flags);
                self.redirected_rutf8(input)?;
                self.ima.cycle_count += cycle_cost;
            }
            _ => self.ima.execute(instruction, input, output)?,
        }
        Ok(())
    }

} 

impl<'a, B: Backend> VisualIMA<'a, B> {

    fn redirected_rfloat<R: BufRead>(&mut self, _: &mut R) -> Result<(), VimaError> {

        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let result: String = loop {
            self.render()?;

            let event = crossterm::event::read().unwrap();
            
            match event {
                crossterm::event::Event::Key(k) if k.kind == crossterm::event::KeyEventKind::Press => match k.code {
                    crossterm::event::KeyCode::Char(c) => self.ima_io.enter_char(c),
                    crossterm::event::KeyCode::Backspace => self.ima_io.delete_char(),
                    crossterm::event::KeyCode::Left => self.ima_io.move_cursor_left(),
                    crossterm::event::KeyCode::Right => self.ima_io.move_cursor_right(),
                    crossterm::event::KeyCode::Enter => {
                        let input = self.ima_io.input();
                        self.ima_io.flush_input();
                        self.ima_io.new_line();
                        break input;
                    },
                    _ => {},
                }
                _ => {},
            }
        };

        self.ima_io_mode = false;

        let mut cursor = std::io::Cursor::new(result);

        self.ima.rfloat(&mut cursor)?;

        Ok(())
    }

    fn redirected_rint<R: BufRead>(&mut self, _: &mut R) -> Result<(), VimaError> {

        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let result: String = loop {
            self.render()?;

            let event = crossterm::event::read().unwrap();
            
            match event {
                crossterm::event::Event::Key(k) if k.kind == crossterm::event::KeyEventKind::Press => match k.code {
                    crossterm::event::KeyCode::Char(c) => self.ima_io.enter_char(c),
                    crossterm::event::KeyCode::Backspace => self.ima_io.delete_char(),
                    crossterm::event::KeyCode::Left => self.ima_io.move_cursor_left(),
                    crossterm::event::KeyCode::Right => self.ima_io.move_cursor_right(),
                    crossterm::event::KeyCode::Enter => {
                        let input = self.ima_io.input();
                        self.ima_io.flush_input();
                        self.ima_io.new_line();
                        break input;
                    },
                    _ => {},
                }
                _ => {},
            }
        };

        self.ima_io_mode = false;

        let mut cursor = std::io::Cursor::new(result);

        self.ima.rint(&mut cursor)?;

        Ok(())
    }

    fn redirected_rutf8<R: BufRead>(&mut self, _: &mut R) -> Result<(), VimaError> {
        
        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let c: char = loop {
            self.render()?;

            let event = crossterm::event::read().unwrap();
            
            match event {
                crossterm::event::Event::Key(k) if k.kind == crossterm::event::KeyEventKind::Press => match k.code {
                    crossterm::event::KeyCode::Char(c) => {
                        self.ima_io.enter_char(c);
                        self.ima_io.flush_input();
                        self.ima_io.new_line();
                        break c
                    },
                    _ => {}
                }
                _ => {},
            }
        };

        self.ima_io_mode = false;

        let mut cursor = std::io::Cursor::new(String::from(c));

        self.ima.rutf8(&mut cursor)?;

        Ok(())
    }
}

