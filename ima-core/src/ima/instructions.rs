/// Created by Virgile HENRY, 2023/09/28


use std::io::{BufRead, Write};

use crate::{ima::{
    IMA,
    address_modes::{
        DVAL,
        GetDval,
        RegisterIndex,
        DADR,
        GetDadr
    },
    error::{
        ImaExecutionError,
        OperationType
    },
    data_type::{
        DataType,
        DataTypeFlag
    },
    zones::memory::Pointer
}, instructions::Instructions};

use super::{
    control_flow::ImaControlFlow,
    options::ImaRunMode,
    zones::program::RunMode
};


impl<RM:RunMode> Instructions for IMA<RM> {
    fn add(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.get_dval(dval)?;
        let v2 = self.registers.get(rm);
        match (v1, v2) {
            (DataType::Float(f1), DataType::Float(f2)) => {
                let res = f1 + f2;
                self.flags.set_ov(res.is_infinite());
                self.flags.set_cmp_float(0.0, res);
                self.registers.set(rm, DataType::Float(res));
            },
            (DataType::Int(i1), DataType::Int(i2)) => {
                let (res, overflow) = i1.overflowing_add(i2);
                self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Add(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn addsp(&mut self, value: u32) -> Result<(), ImaExecutionError> {
        self.sp = self.sp.offset(value as i32).ok_or(ImaExecutionError::StackOverflow)?;
        Ok(())
    }

    fn beq(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.eq() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bge(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.ge() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bgt(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.gt() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn ble(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.le() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn blt(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.lt() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bne(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.ne() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bov(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        if !self.flags.ov() { return Ok(()) }
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bra(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => Ok(self.code.set_pc(addr)),
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn bsr(&mut self, dval: DVAL) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::CodeAddr(addr) => {
                self.sp = self.sp.offset(2).ok_or(ImaExecutionError::StackOverflow)?;
                self.memory.set_stack(self.sp.offset(-1).unwrap(), DataType::CodeAddr(self.code.pc()))?;
                self.memory.set_stack(self.sp, DataType::MemAddr(Pointer::Stack(self.lb)))?;
                self.lb = self.sp;
                self.code.set_pc(addr);
                Ok(())
            },
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::CodeAddr, found: v.into() })
        }
    }

    fn clk(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.ima_start_time);
        self.registers.set(RegisterIndex(0), DataType::Float(elapsed.as_secs_f32()));
    }

    fn cmp(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.get_dval(dval)?;
        let v2 = self.registers.get(rm);
        match (v1, v2) {
            (DataType::Float(f1), DataType::Float(f2)) => {
                self.flags.set_cmp_float(f1, f2);
            },
            (DataType::Int(i1), DataType::Int(i2)) => {
                self.flags.set_cmp_int(i1, i2);
            },
            (DataType::MemAddr(m1), DataType::MemAddr(m2)) => {
                self.flags.set_cmp_ptr(m1, m2);
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Cmp(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn del(&mut self, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let addr = self.registers.get(rm);
        match addr {
            DataType::MemAddr(Pointer::Heap(addr)) => {
                match self.memory.free(addr) {
                    Some(_) => {},
                    None => self.flags.set_ov(true),
                };
                Ok(())
            },
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::MemAddr, found: addr.into() }),
        }
    }

    fn div(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.registers.get(rm);
        let v2 = self.get_dval(dval)?;
        match (v1, v2) {
            (DataType::Float(f1), DataType::Float(f2)) => {
                let res = f1 / f2;
                self.flags.set_ov(res.is_infinite());
                self.flags.set_cmp_float(0.0, res);
                self.registers.set(rm, DataType::Float(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Div(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn error(&mut self) {
        // todo : this could be done more gracefully
        self.control_flow = ImaControlFlow::Error;
    }

    fn float(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::Int(i) => {
                // todo : some error handling checks ? no idea how rust handles it
                self.registers.set(rm, DataType::Float(i as f32));
            },
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Int, found: v.into() }),
        }
        Ok(())
    }

    fn fma(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.get_dval(dval)?;
        let v2 = self.registers.get(rm);
        let v3 = self.registers.get(RegisterIndex(0));
        match (v1, v2, v3) {
            (DataType::Float(f1), DataType::Float(f2), DataType::Float(f3)) => {
                let res = f1 * f2 + f3;
                self.flags.set_ov(res.is_infinite());
                self.flags.set_cmp_float(0.0, res);
                self.registers.set(rm, DataType::Float(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Fma(v1.into(), v2.into(), v3.into()))),
        }
        Ok(())
    }

    fn halt(&mut self) {
        self.control_flow = ImaControlFlow::Halt;
    }

    fn int(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::Float(f) => {
                // todo : some error handling checks ? no idea how rust handles it
                self.registers.set(rm, DataType::Int(f as i32));
            },
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Float, found: v.into() }),
        }
        Ok(())
    }

    fn lea(&mut self, dadr: DADR, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let addr = self.get_dadr(dadr)?;
        self.registers.set(rm, DataType::MemAddr(addr));
        Ok(())
    }

    fn load(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::Int(i) => self.flags.set_cmp_int(0, i),
            DataType::Float(f) => self.flags.set_cmp_float(0.0, f),
            DataType::MemAddr(a) => self.flags.set_cmp_ptr(Pointer::Null, a),
            _ => {},
        }
        self.registers.set(rm, v);
        Ok(())
    }

    fn mul(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.get_dval(dval)?;
        let v2 = self.registers.get(rm);
        match (v1, v2) {
            (DataType::Float(f1), DataType::Float(f2)) => {
                let res = f1 * f2;
                self.flags.set_ov(res.is_infinite());
                self.flags.set_cmp_float(0.0, res);
                self.registers.set(rm, DataType::Float(res));
            },
            (DataType::Int(i1), DataType::Int(i2)) => {
                let (res, overflow) = i1.overflowing_mul(i2);
                self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Mul(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn new(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::Int(i) => {
                let ptr = match self.memory.allocate(i as usize) {
                    Some(addr) => Pointer::Heap(addr),
                    None => {
                        self.flags.set_ov(true);
                        Pointer::Null
                    }
                };
                self.registers.set(rm, DataType::MemAddr(ptr));
                Ok(())
            },
            _ => Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Int, found: v.into() }),
        }
    }

    fn opp(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.get_dval(dval)?;
        match v {
            DataType::Float(f) => {
                self.flags.set_cmp_float(0.0, -f);
                self.registers.set(rm, DataType::Float(-f));
            },
            DataType::Int(i) => {
                let (res, overflow) = i.overflowing_neg();
                // todo : in the docs, opp does not set overflow.
                self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Opp(v.into()))),
        }
        Ok(())
    }

    fn pea(&mut self, dadr: DADR) -> Result<(), ImaExecutionError> {
        let addr = self.get_dadr(dadr)?;
        self.sp = self.sp.offset(1).ok_or(ImaExecutionError::StackOverflow)?;
        self.memory.set_stack(self.sp, DataType::MemAddr(addr))?;
        Ok(())
    }

    fn pop(&mut self, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        // the stack pointer always points on the stack, so safe to unwrap
        let v = self.memory.get_stack(self.sp).unwrap();
        match v {
            DataType::Int(i) => self.flags.set_cmp_int(0, i),
            DataType::Float(f) => self.flags.set_cmp_float(0.0, f),
            DataType::MemAddr(a) => self.flags.set_cmp_ptr(Pointer::Null, a),
            _ => {},
        }
        self.registers.set(rm, v);
        self.sp = self.sp.offset(-1).ok_or(ImaExecutionError::StackUnderflow)?;
        Ok(())
    }

    fn push(&mut self, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(rm);
        match v {
            DataType::Int(i) => self.flags.set_cmp_int(0, i),
            DataType::Float(f) => self.flags.set_cmp_float(0.0, f),
            DataType::MemAddr(a) => self.flags.set_cmp_ptr(Pointer::Null, a),
            _ => {},
        }
        self.sp = self.sp.offset(1).ok_or(ImaExecutionError::StackOverflow)?;
        self.memory.set_stack(self.sp, v)?;
        Ok(())
    }

    fn quo(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.registers.get(rm);
        let v2 = self.get_dval(dval)?;
        match (v1, v2) {
            (DataType::Int(i1), DataType::Int(i2)) => {
                if i2 == 0 {
                    self.flags.set_ov(true);
                }
                else {
                    let res = i1 / i2;
                    self.flags.set_cmp_int(0, res);
                    self.registers.set(rm, DataType::Int(res));
                }
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Quo(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn rem(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.registers.get(rm);
        let v2 = self.get_dval(dval)?;
        match (v1, v2) {
            (DataType::Int(i1), DataType::Int(i2)) => {
                if i2 == 0 {
                    self.flags.set_ov(true);
                }
                else {
                    let res = i1 % i2;
                    self.flags.set_cmp_int(0, res);
                    self.registers.set(rm, DataType::Int(res));
                }
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Rem(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn rfloat<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError> {
        // wait for user input
        let mut user_input = String::new();

        match input.read_line(&mut user_input) {
            Ok(_) => {},
            Err(e) => return Err(ImaExecutionError::FailedToReadInput(e))
        }

        match user_input.trim().parse() {
            Ok(num) => {
                self.flags.set_cmp_float(0.0, num);
                self.registers.set(RegisterIndex(1), DataType::Float(num));
            },
            Err(_) => self.flags.set_ov(true),
        };

        Ok(())
    }

    fn rint<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError> {
        // wait for user input
        let mut user_input = String::new();

        match input.read_line(&mut user_input) {
            Ok(_) => {},
            Err(e) => return Err(ImaExecutionError::FailedToReadInput(e))
        }

        match user_input.trim().parse() {
            Ok(num) => {
                self.flags.set_cmp_int(0, num);
                self.registers.set(RegisterIndex(1), DataType::Int(num));
            },
            Err(_) => self.flags.set_ov(true),
        };

        Ok(())
    }

    fn rts(&mut self) -> Result<(), ImaExecutionError> {
        self.code.set_pc({
            let addr = self.lb.offset(-1).ok_or(ImaExecutionError::StackUnderflow)?;
            let v = self.memory.get_stack(addr).ok_or(ImaExecutionError::InvalidMemoryAddress(Pointer::Stack(addr)))?;
            match v {
                DataType::CodeAddr(addr) => addr,
                _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::MemAddr, found: v.into() }),
            }
        });
        self.sp = self.lb.offset(-2).ok_or(ImaExecutionError::StackUnderflow)?;
        self.lb = {
            let addr = self.lb;
            let v = self.memory.get_stack(addr).ok_or(ImaExecutionError::InvalidMemoryAddress(Pointer::Stack(addr)))?;
            match v {
                DataType::MemAddr(Pointer::Stack(addr)) => addr,
                _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::MemAddr, found: v.into() }),
            }
        };
        Ok(())
    }

    fn rutf8<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError> {
        let mut buf = [0u8; 4];
        input.read_exact(&mut buf).map_err(|e| ImaExecutionError::FailedToReadInput(e))?;

        let v = i32::from_ne_bytes(buf);
        self.registers.set(RegisterIndex(1), DataType::Int(v));

        Ok(())
    }

    fn sclk(&mut self) {
        use chrono::TimeZone;
        // Create a Utc DateTime for January 1, 2001, 00:00:00
        let start_date = chrono::Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).single().unwrap();

        // Get the current Utc DateTime or any other DateTime you want to measure the duration from
        let current_date = chrono::Utc::now(); // Replace this with your desired instant

        // Calculate the duration between the two DateTimes
        let duration = current_date.signed_duration_since(start_date);

        // Get the total number of seconds in the duration
        let seconds = duration.num_seconds();

        // finally write to R1
        self.registers.set(RegisterIndex(1), DataType::Int(seconds as i32));
    }

    fn seq(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.eq().into()))
    }

    fn setround_downward(&mut self) {
        // todo
    }

    fn setround_tonearest(&mut self) {
        // todo
    }

    fn setround_towardzero(&mut self) {
        // todo
    }

    fn setround_upward(&mut self) {
        // todo
    }

    fn sge(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.ge().into()))
    }

    fn sgt(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.gt().into()))
    }

    fn shl(&mut self, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(rm);
        match v {
            DataType::Int(i) => {
                let (res, overflow) = i.overflowing_shl(1);
                self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Shl(v.into()))),
        }
        Ok(())
    }

    fn shr(&mut self, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(rm);
        match v {
            DataType::Int(i) => {
                let (res, _overflow) = i.overflowing_shr(1);
                // self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Shr(v.into()))),
        }
        Ok(())
    }

    fn sle(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.le().into()))
    }

    fn slt(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.lt().into()))
    }

    fn sne(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.ne().into()))
    }

    fn sov(&mut self, rm: RegisterIndex) {
        self.registers.set(rm, DataType::Int(self.flags.ov().into()))
    }

    fn store(&mut self, rm: RegisterIndex, dadr: DADR) -> Result<(), ImaExecutionError> {
        let addr = self.get_dadr(dadr)?;
        let v = self.registers.get(rm);
        match v {
            DataType::Int(i) => self.flags.set_cmp_int(0, i),
            DataType::Float(f) => self.flags.set_cmp_float(0.0, f),
            DataType::MemAddr(a) => self.flags.set_cmp_ptr(Pointer::Null, a),
            _ => {},
        }
        self.memory.set(addr, v)
    }

    fn sub(&mut self, dval: DVAL, rm: RegisterIndex) -> Result<(), ImaExecutionError> {
        let v1 = self.registers.get(rm);
        let v2 = self.get_dval(dval)?;
        match (v1, v2) {
            (DataType::Float(f1), DataType::Float(f2)) => {
                let res = f1 - f2;
                self.flags.set_ov(res.is_infinite());
                self.flags.set_cmp_float(0.0, res);
                self.registers.set(rm, DataType::Float(res));
            },
            (DataType::Int(i1), DataType::Int(i2)) => {
                let (res, overflow) = i1.overflowing_sub(i2);
                self.flags.set_ov(overflow);
                self.flags.set_cmp_int(0, res);
                self.registers.set(rm, DataType::Int(res));
            },
            _ => return Err(ImaExecutionError::InvalidOperation(OperationType::Sub(v1.into(), v2.into()))),
        }
        Ok(())
    }

    fn subsp(&mut self, value: u32) -> Result<(), ImaExecutionError> {
        self.sp = self.sp.offset(-(value as i32)).ok_or(ImaExecutionError::StackUnderflow)?;
        Ok(())
    }

    fn tsto(&mut self, value: u32) {
        self.flags.set_ov(self.sp.as_index() + value as usize > self.memory.stack_size())
    }

    fn wfloat<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(RegisterIndex(1));
        match v {
            DataType::Float(f) => {
                output.write(format!("{}", f).as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
                output.flush().map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
            },
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Float, found: v.into() }),
        }
        Ok(())
    }

    fn wfloatx<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(RegisterIndex(1));
        match v {
            DataType::Float(f) => {
                output.write(format!("{}", f).as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
                if self.run_mode == ImaRunMode::WriteNewLines {
                    output.write(format!("\n").as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
                }
                output.flush().map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
            },
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Float, found: v.into() }),
        }
        Ok(())
    }

    fn wint<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError> {
        let v = self.registers.get(RegisterIndex(1));
        match v {
            DataType::Int(i) => {
                output.write(format!("{}", i).as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
                if self.run_mode == ImaRunMode::WriteNewLines {
                    output.write(format!("\n").as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
                }
                output.flush().map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
            },
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Int, found: v.into() }),
        }
        Ok(())
    }

    fn wnl<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError> {
        output.write(format!("\n").as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
        output.flush().map_err(|e| ImaExecutionError::FailedToWriteIO(e))
    }

    fn wstr<W: Write>(&mut self, output: &mut W, string: String) -> Result<(), ImaExecutionError> {
        output.write(string.as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
        if self.run_mode == ImaRunMode::WriteNewLines {
            output.write(format!("\n").as_bytes()).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
        }
        output.flush().map_err(|e| ImaExecutionError::FailedToWriteIO(e))
    }

    fn wutf8<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError> {
        let bytes = match self.registers.get(RegisterIndex(1)) {
            DataType::Int(i) => {
                let u: u32 = unsafe { std::mem::transmute(i) };
                u.to_be_bytes()
            }
            _ => return Err(ImaExecutionError::InvalidDataType { expected: DataTypeFlag::Int, found: self.registers.get(RegisterIndex(1)).into() }),
        };
        output.write(&bytes).map_err(|e| ImaExecutionError::FailedToWriteIO(e))?;
        Ok(())
    }
        
}