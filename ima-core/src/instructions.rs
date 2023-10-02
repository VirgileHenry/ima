/// Created by Virgile HENRY, 2023/09/28

use std::{
    fmt::Display,
    io::{
        Write,
        BufRead
    }
};

use crate::ima::{
    address_modes::{
        DVAL,
        RegisterIndex,
        DADR
    },
    error::ImaExecutionError,
};

type Rm = RegisterIndex;

/// The instruction set of the IMA machine.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    // data transfer
    LOAD(DVAL, Rm),
    STORE(Rm, DADR),
    PUSH(Rm),
    POP(Rm),
    LEA(DADR, Rm),
    PEA(DADR),

    // memory allocations
    NEW(DVAL, Rm),
    DEL(Rm),

    // value comparison
    CMP(DVAL, Rm),

    // arithmetic (int / floats)
    ADD(DVAL, Rm),
    SUB(DVAL, Rm),
    MUL(DVAL, Rm),
    OPP(DVAL, Rm),

    // arithmetic (ints)
    QUO(DVAL, Rm),
    REM(DVAL, Rm),
    SEQ(Rm),
    SGT(Rm),
    SGE(Rm),
    SOV(Rm),
    SNE(Rm),
    SLT(Rm),
    SLE(Rm),
    SHL(Rm),
    SHR(Rm),

    // arithmetic (floats)
    DIV(DVAL, Rm),
    FMA(DVAL, Rm),

    // conversion (int / floats)
    FLOAT(DVAL, Rm),
    INT(DVAL, Rm),

    // floats managment
    SETROUND_TONEAREST,
    SETROUND_UPWARD,
    SETROUND_DOWNWARD,
    SETROUND_TOWARDZERO,

    // control flow
    BRA(DVAL),
    BEQ(DVAL),
    BGT(DVAL),
    BGE(DVAL),
    BOV(DVAL),
    BNE(DVAL),
    BLT(DVAL),
    BLE(DVAL),
    BSR(DVAL),
    RTS,

    // i/o
    RINT,
    RFLOAT,
    WINT,
    WFLOAT,
    WFLOATX,
    WSTR(String),
    WNL,
    RUTF8,
    WUTF8,

    // miscelaneous
    ADDSP(u32),
    SUBSP(u32),
    TSTO(u32),
    HALT,
    ERROR,
    SCLK,
    CLK,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Instruction::ADD(dval, rm) => write!(f, "ADD {}, {}", dval, rm),
            Instruction::ADDSP(value) => write!(f, "ADDSP {}", value),
            Instruction::BEQ(dval) => write!(f, "BEQ {}", dval),
            Instruction::BGE(dval) => write!(f, "BGE {}", dval),
            Instruction::BGT(dval) => write!(f, "BGT {}", dval),
            Instruction::BLE(dval) => write!(f, "BLE {}", dval),
            Instruction::BLT(dval) => write!(f, "BLT {}", dval),
            Instruction::BOV(dval) => write!(f, "BOV {}", dval),
            Instruction::BRA(dval) => write!(f, "BRA {}", dval),
            Instruction::BNE(dval) => write!(f, "BNE {}", dval),
            Instruction::BSR(dval) => write!(f, "BSR {}", dval),
            Instruction::CMP(dval, rm) => write!(f, "CMP {}, {}", dval, rm),
            Instruction::DEL(rm) => write!(f, "DEL {}", rm),
            Instruction::DIV(dval, rm) => write!(f, "DIV {}, {}", dval, rm),
            Instruction::ERROR => write!(f, "ERROR"),
            Instruction::FMA(dval, rm) => write!(f, "FMA {}, {}", dval, rm),
            Instruction::FLOAT(dval, rm) => write!(f, "FLOAT {}, {}", dval, rm),
            Instruction::HALT => write!(f, "HALT"),
            Instruction::INT(dval, rm) => write!(f, "INT {}, {}", dval, rm),
            Instruction::LEA(dadr, rm) => write!(f, "LEA {}, {}", dadr, rm),
            Instruction::LOAD(dval, rm) => write!(f, "LOAD {}, {}", dval, rm),
            Instruction::MUL(dval, rm) => write!(f, "MUL {}, {}", dval, rm),
            Instruction::NEW(dval, rm) => write!(f, "NEW {}, {}", dval, rm),
            Instruction::OPP(dval, rm) => write!(f, "OPP {}, {}", dval, rm),
            Instruction::PEA(dadr) => write!(f, "PEA {}", dadr),
            Instruction::POP(rm) => write!(f, "POP {}", rm),
            Instruction::PUSH(rm) => write!(f, "PUSH {}", rm),
            Instruction::QUO(dval, rm) => write!(f, "QUO {}, {}", dval, rm),
            Instruction::REM(dval, rm) => write!(f, "REM {}, {}", dval, rm),
            Instruction::RTS => write!(f, "RTS"),
            Instruction::SEQ(rm) => write!(f, "SEQ {}", rm),
            Instruction::SETROUND_DOWNWARD => write!(f, "SETROUND_DOWNWARD"),
            Instruction::SETROUND_TONEAREST => write!(f, "SETROUND_TONEAREST"),
            Instruction::SETROUND_TOWARDZERO => write!(f, "SETROUND_TOWARDZERO"),
            Instruction::SETROUND_UPWARD => write!(f, "SETROUND_UPWARD"),
            Instruction::SGE(rm) => write!(f, "SGE {}", rm),
            Instruction::SGT(rm) => write!(f, "SGT {}", rm),
            Instruction::SHL(rm) => write!(f, "SHL {}", rm),
            Instruction::SHR(rm) => write!(f, "SHR {}", rm),
            Instruction::SLE(rm) => write!(f, "SLE {}", rm),
            Instruction::SLT(rm) => write!(f, "SLT {}", rm),
            Instruction::SOV(rm) => write!(f, "SOV {}", rm),
            Instruction::SNE(rm) => write!(f, "SNE {}", rm),
            Instruction::STORE(rm, dadr) => write!(f, "STORE {}, {}", rm, dadr),
            Instruction::SUB(dval, rm) => write!(f, "SUB {}, {}", dval, rm),
            Instruction::SUBSP(value) => write!(f, "SUBSP {}", value),
            Instruction::TSTO(value) => write!(f, "TSTO {}", value),
            Instruction::WFLOAT => write!(f, "WFLOAT"),
            Instruction::WFLOATX => write!(f, "WFLOATX"),
            Instruction::WINT => write!(f, "WINT"),
            Instruction::WNL => write!(f, "WNL"),
            Instruction::WSTR(string) => write!(f, "WSTR \"{}\"", string.replace('\"', "\"\"")),
            Instruction::WUTF8 => write!(f, "WUTF8"),
            Instruction::RINT => write!(f, "RINT"),
            Instruction::RFLOAT => write!(f, "RFLOAT"),
            Instruction::RUTF8 => write!(f, "RUTF8"),
            Instruction::SCLK => write!(f, "SCLK"),
            Instruction::CLK => write!(f, "CLK"),
        }
    }
}

/// Trait for any object that can execute IMA instructions.
pub trait Instructions {
    /// Load the value dval in the register Rm.
    /// 
    /// Rm <- V\[dval\]
    /// 
    /// CC: CP
    fn load(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Store the value in the register Rm at the address dadr.
    /// 
    /// A\[dadr\] <- V\[Rm\]
    /// 
    /// CC: CP
    fn store(&mut self, rm: Rm, dadr: DADR) -> Result<(), ImaExecutionError>;
    /// Push the value in the register Rm on the stack.
    /// 
    /// V\[SP\]+1 <- V\[Rm\],
    /// 
    /// SP <- V\[SP\] + 1
    /// 
    /// CC: CP
    fn push(&mut self, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Pop the value on the stack in the register Rm.
    /// 
    /// Rm <- V\[V\[SP\]\],
    /// 
    /// SP <- V\[SP\] - 1
    /// 
    /// CC: CP
    fn pop(&mut self, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Load the address dadr in the register Rm.
    /// 
    /// Rm <- A\[dadr\]
    /// 
    /// CC: CP
    fn lea(&mut self, dadr: DADR, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Push the address dadr on the stack
    /// 
    /// V\[SP\]+1 <- A\[dadr\],
    /// 
    /// SP <- V\[SP\] + 1
    fn pea(&mut self, dadr: DADR) -> Result<(), ImaExecutionError>;
    /// Allocate a new memory zone of size dval and store the address in the register Rm.
    /// The address in Rm is the address of the first allocated word.
    fn new(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Free the memory zone at the address in the register Rm.
    /// 
    /// CC: OV (invalid address)
    fn del(&mut self, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Compares the values in the register Rm and the value dval.
    /// 
    /// CC: CP
    fn cmp(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs an addition between the value in the register Rm and the value dval.
    /// This can be done on integers or floats.
    /// 
    /// Rm <- V\[Rm\] + V\[dval\]
    /// 
    /// CC: CP
    fn add(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a subtraction between the value in the register Rm and the value dval.
    /// This can be done on integers or floats.
    /// 
    /// Rm <- V\[Rm\] - V\[dval\]
    /// 
    /// CC: OV, CP
    fn sub(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a multiplication between the value in the register Rm and the value dval.
    /// This can be done on integers or floats.
    /// 
    /// Rm <- V\[Rm\] * V\[dval\]
    /// 
    /// CC: OV, CP
    fn mul(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs an opposite on the value in the register Rm.
    /// This can be done on integers or floats.
    /// 
    /// Rm <- -V\[Rm\]
    /// 
    /// CC: OV, CP
    fn opp(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a Euclidian division between the value in the register Rm and the value dval.
    /// This can only be done on integers.
    /// 
    /// Rm <- V\[Rm\] / V\[dval\]
    /// 
    /// CC: CP
    fn quo(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a Euclidian modulo between the value in the register Rm and the value dval.
    /// This can only be done on integers.
    /// 
    /// Rm <- V\[Rm\] % V\[dval\]
    /// 
    /// CC: CP
    fn rem(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Checks the EQ flag. if EQ is true, the value in the register Rm is set to 1, else 0.
    fn seq(&mut self, rm: Rm);
    /// Checks the GT flag. if GT is true, the value in the register Rm is set to 1, else 0.
    fn sgt(&mut self, rm: Rm);
    /// Checks the GE flag. if GE is true, the value in the register Rm is set to 1, else 0.
    fn sge(&mut self, rm: Rm);
    /// Checks the OV flag. if OV is true, the value in the register Rm is set to 1, else 0.
    fn sov(&mut self, rm: Rm);
    /// Checks the NE flag. if NE is true, the value in the register Rm is set to 1, else 0.
    fn sne(&mut self, rm: Rm);
    /// Checks the LT flag. if LT is true, the value in the register Rm is set to 1, else 0.
    fn slt(&mut self, rm: Rm);
    /// Checks the LE flag. if LE is true, the value in the register Rm is set to 1, else 0.
    fn sle(&mut self, rm: Rm);
    /// Shifts the value in the register Rm to the left by 1.
    /// 
    /// Rm <- V\[Rm\] << 1
    /// 
    /// CC: OV, CP
    fn shl(&mut self, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Shifts the value in the register Rm to the right by 1.
    /// 
    /// Rm <- V\[Rm\] >> 1
    /// 
    /// CC: CP
    fn shr(&mut self, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a division between the value in the register Rm and the value dval.
    /// This can only be done on floats.
    /// 
    /// Rm <- V\[Rm\] / V\[dval\]
    /// 
    /// CC: CP
    fn div(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Performs a fused multiply-add between the value in the register Rm and the value dval.
    /// This can only be done on floats.
    /// 
    /// Rm <- V\[Rm\] * V\[dval\] + V\[R1\]
    /// 
    /// CC: CP
    fn fma(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Converts the value in the register Rm to a float.
    /// This can only be done on integers.
    /// 
    /// Rm <- float(V\[dval\])
    /// 
    /// CC: OV (unable to convert to float)
    fn float(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Converts the value in the register Rm to an integer.
    /// This can only be done on floats.
    /// 
    /// Rm <- int(V\[dval\])
    /// 
    /// CC: OV (unable to convert to int)
    fn int(&mut self, dval: DVAL, rm: Rm) -> Result<(), ImaExecutionError>;
    /// Sets the floating point operation rounding mode to nearest.
    fn setround_tonearest(&mut self);
    /// Sets the floating point operation rounding mode to upward.
    fn setround_upward(&mut self);
    /// Sets the floating point operation rounding mode to downward.
    fn setround_downward(&mut self);
    /// Sets the floating point operation rounding mode to toward zero.
    fn setround_towardzero(&mut self);
    /// Unconditional branch to the address dval.
    /// 
    /// PC <- V\[dval\]
    fn bra(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the EQ flag is true.
    /// 
    /// if EQ: PC <- V\[dval\]
    fn beq(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the GT flag is true.
    /// 
    /// if GT: PC <- V\[dval\]
    fn bgt(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the GE flag is true.
    /// 
    /// if GE: PC <- V\[dval\]
    fn bge(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the OV flag is true.
    /// 
    /// if OV: PC <- V\[dval\]
    fn bov(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the NE flag is true.
    /// 
    /// if NE: PC <- V\[dval\]
    fn bne(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the LT flag is true.
    /// 
    /// if LT: PC <- V\[dval\]
    fn blt(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval if the LE flag is true.
    /// 
    /// if LE: PC <- V\[dval\]
    fn ble(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Branch to the address dval, performing operations for a function call:
    /// 
    /// SP <- V\[SP\] + 2,
    /// 
    /// V\[SP\]-1 <- PC,
    /// 
    /// V\[SP\] <- V\[LB\],
    /// 
    /// LB <- V\[SP\],
    /// 
    /// PC <- V\[dval\]
    fn bsr(&mut self, dval: DVAL) -> Result<(), ImaExecutionError>;
    /// Return from a function call:
    /// 
    /// PC <- C\[V\[LB\]-1\],
    /// 
    /// SP <- V\[LB\] - 2,
    /// 
    /// LB <- C\[V\[LB\]\]
    fn rts(&mut self) -> Result<(), ImaExecutionError>;
    /// Read an integer from the standard input and store it in the register R1.
    /// This wait a newline character before completing.
    fn rint<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError>;
    /// Read a float from the standard input and store it in the register R1.
    /// This wait a newline character before completing.
    fn rfloat<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError>;
    /// Write the value in the register R1 to the standard output as an integer.
    fn wint<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError>;
    /// Write the value in the register R1 to the standard output as a float.
    fn wfloat<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError>;
    /// Write the value in the register R1 to the standard output as a float, in hexadecimal.
    fn wfloatx<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError>;
    /// Write the string to the standard output.
    fn wstr<W: Write>(&mut self, output: &mut W, string: String) -> Result<(), ImaExecutionError>;
    /// Write a newline character to the standard output.
    fn wnl<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError>;
    /// Read a UTF-8 character from the standard input and store it's code in the register R1.
    /// This will consume any unconsumed character and will not wait for a newline character.
    fn rutf8<R: BufRead>(&mut self, input: &mut R) -> Result<(), ImaExecutionError>;
    /// Write the UTF-8 character in the register R1 to the standard output.
    fn wutf8<W: Write>(&mut self, output: &mut W) -> Result<(), ImaExecutionError>;
    /// Add the value to the stack pointer.
    /// 
    /// SP <- V\[SP\] + value
    fn addsp(&mut self, value: u32) -> Result<(), ImaExecutionError>;
    /// Subtract the value from the stack pointer.
    /// 
    /// SP <- V\[SP\] - value
    fn subsp(&mut self, value: u32) -> Result<(), ImaExecutionError>;
    /// Test the remaining size of the stack.
    ///
    /// CC: OV if V\[SP\] + value > V\[GB\] + N
    fn tsto(&mut self, value: u32);
    /// Stop the machine.
    fn halt(&mut self);
    /// Stop the machine with an error status
    fn error(&mut self);
    /// Load in R1 the number of seconds as an integer between 01/01/2001 : 00:00 and the start of the machine.
    fn sclk(&mut self);
    /// Load in R1 the number of seconds as a float between the start of the machine and now.
    fn clk(&mut self);
}