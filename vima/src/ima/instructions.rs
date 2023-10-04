use std::io::{BufRead, Write};

use ima_core::complete::*;
use super::VisualIMA;

use ratatui::prelude::Backend;

impl<'a, B: Backend> VisualIMA<'a, B> {
    pub fn execute<R: BufRead, W: Write>(&mut self, instruction: Instruction, input: &mut R, output: &mut W) -> Result<(), ImaExecutionError> {
        let cycle_cost = instruction.cycle_cost(&self.ima.flags);
        match instruction {
            Instruction::LOAD(dval, rm) => self.ima.load(dval, rm)?,
            Instruction::STORE(rm, dadr) => self.ima.store(rm, dadr)?,
            Instruction::PUSH(rm) => self.ima.push(rm)?,
            Instruction::POP(rm) => self.ima.pop(rm)?,
            Instruction::LEA(dadr, rm) => self.ima.lea(dadr, rm)?,
            Instruction::PEA(dadr) => self.ima.pea(dadr)?,
            Instruction::NEW(dval, rm) => self.ima.new(dval, rm)?,
            Instruction::DEL(rm) => self.ima.del(rm)?,
            Instruction::CMP(dval, rm) => self.ima.cmp(dval, rm)?,
            Instruction::ADD(dval, rm) => self.ima.add(dval, rm)?,
            Instruction::SUB(dval, rm) => self.ima.sub(dval, rm)?,
            Instruction::MUL(dval, rm) => self.ima.mul(dval, rm)?,
            Instruction::OPP(dval, rm) => self.ima.opp(dval, rm)?,
            Instruction::QUO(dval, rm) => self.ima.quo(dval, rm)?,
            Instruction::REM(dval, rm) => self.ima.rem(dval, rm)?,
            Instruction::SEQ(rm) => self.ima.seq(rm),
            Instruction::SGT(rm) => self.ima.sgt(rm),
            Instruction::SGE(rm) => self.ima.sge(rm),
            Instruction::SOV(rm) => self.ima.sov(rm),
            Instruction::SNE(rm) => self.ima.sne(rm),
            Instruction::SLT(rm) => self.ima.slt(rm),
            Instruction::SLE(rm) => self.ima.sle(rm),
            Instruction::SHL(rm) => self.ima.shl(rm)?,
            Instruction::SHR(rm) => self.ima.shr(rm)?,
            Instruction::DIV(dval, rm) => self.ima.div(dval, rm)?,
            Instruction::FMA(dval, rm) => self.ima.fma(dval, rm)?,
            Instruction::FLOAT(dval, rm) => self.ima.float(dval, rm)?,
            Instruction::INT(dval, rm) => self.ima.int(dval, rm)?,
            Instruction::SETROUND_TONEAREST => self.ima.setround_tonearest(),
            Instruction::SETROUND_UPWARD => self.ima.setround_upward(),
            Instruction::SETROUND_DOWNWARD => self.ima.setround_downward(),
            Instruction::SETROUND_TOWARDZERO => self.ima.setround_towardzero(),
            Instruction::BRA(dval) => self.ima.bra(dval)?,
            Instruction::BEQ(dval) => self.ima.beq(dval)?,
            Instruction::BGT(dval) => self.ima.bgt(dval)?,
            Instruction::BGE(dval) => self.ima.bge(dval)?,
            Instruction::BOV(dval) => self.ima.bov(dval)?,
            Instruction::BNE(dval) => self.ima.bne(dval)?,
            Instruction::BLT(dval) => self.ima.blt(dval)?,
            Instruction::BLE(dval) => self.ima.ble(dval)?,
            Instruction::BSR(dval) => self.ima.bsr(dval)?,
            Instruction::RTS => self.ima.rts()?,
            Instruction::RINT => self.redirected_rint(input)?,
            Instruction::RFLOAT => self.redirected_rfloat(input)?,
            Instruction::WINT => self.ima.wint(output)?,
            Instruction::WFLOAT => self.ima.wfloat(output)?,
            Instruction::WFLOATX => self.ima.wfloatx(output)?,
            Instruction::WSTR(string) => self.ima.wstr(output, string)?,
            Instruction::WNL => self.ima.wnl(output)?,
            Instruction::RUTF8 => self.redirected_rutf8(input)?,
            Instruction::WUTF8 => self.ima.wutf8(output)?,
            Instruction::ADDSP(value) => self.ima.addsp(value)?,
            Instruction::SUBSP(value) => self.ima.subsp(value)?,
            Instruction::TSTO(value) => self.ima.tsto(value),
            Instruction::HALT => self.ima.halt(),
            Instruction::ERROR => self.ima.error(),
            Instruction::SCLK => self.ima.sclk(),
            Instruction::CLK => self.ima.clk(),
        }
        self.ima.cycle_count += cycle_cost;
        Ok(())
    }

} 

impl<'a, B: Backend> VisualIMA<'a, B> {

    fn redirected_rfloat<R: BufRead>(&mut self, _: &mut R) -> Result<(), ImaExecutionError> {

        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let result: String = loop {
            let _ = self.render();

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

        self.ima.rfloat(&mut cursor)
    }

    fn redirected_rint<R: BufRead>(&mut self, _: &mut R) -> Result<(), ImaExecutionError> {

        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let result: String = loop {
            let _ = self.render();

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

        self.ima.rint(&mut cursor)
    }

    fn redirected_rutf8<R: BufRead>(&mut self, _: &mut R) -> Result<(), ImaExecutionError> {
        
        // any things in input is flushed in the lines
        self.ima_io.flush_input();
        self.ima_io_mode = true;
        
        let c: char = loop {
            let _ = self.render();

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

        self.ima.rutf8(&mut cursor)
    }
}

