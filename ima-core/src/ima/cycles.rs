use crate::complete::{Instruction, DVAL, DADR, Flags};

pub trait CycleCost {
    fn cycle_cost(&self, flags: &Flags) -> usize;
}

impl CycleCost for DADR {
    fn cycle_cost(&self, _flags: &Flags) -> usize {
        match &self {
            DADR::OffsetIndirect { .. } => 4,
            DADR::OffsetAndDisplacedIndirect { .. } => 5,
        }
    }
}

impl CycleCost for DVAL {
    fn cycle_cost(&self, flags: &Flags) -> usize {
        match &self {
            DVAL::DADR(dadr) => dadr.cycle_cost(flags),
            DVAL::Immediate(_) => 2,
            DVAL::Label(_) => 2,
            DVAL::Register(_) => 0,
        }
    }
}

impl CycleCost for Instruction {
    fn cycle_cost(&self, flags: &Flags) -> usize {
        match &self {
            Instruction::ADD(dval, _) => 2 + dval.cycle_cost(flags),
            Instruction::ADDSP(_imm) => 4,
            Instruction::BEQ(dval) => dval.cycle_cost(flags) + if flags.eq() {5} else {4},
            Instruction::BGE(dval) => dval.cycle_cost(flags) + if flags.ge() {5} else {4},
            Instruction::BGT(dval) => dval.cycle_cost(flags) + if flags.gt() {5} else {4},
            Instruction::BLE(dval) => dval.cycle_cost(flags) + if flags.le() {5} else {4},
            Instruction::BLT(dval) => dval.cycle_cost(flags) + if flags.lt() {5} else {4},
            Instruction::BNE(dval) => dval.cycle_cost(flags) + if flags.ne() {5} else {4},
            Instruction::BOV(dval) => dval.cycle_cost(flags) + if flags.ov() {5} else {4},
            Instruction::BRA(dval) => 5 + dval.cycle_cost(flags),
            Instruction::BSR(dval) => 9 + dval.cycle_cost(flags),
            Instruction::CLK => 16,
            Instruction::CMP(dval, _rm) => 2 + dval.cycle_cost(flags),
            Instruction::DEL(_rm) => 16,
            Instruction::DIV(dval, _rm) => 40 + dval.cycle_cost(flags),
            Instruction::ERROR => 1,
            Instruction::FLOAT(dval, _rm) => 4 + dval.cycle_cost(flags),
            Instruction::FMA(dval, _rm) => 21 + dval.cycle_cost(flags),
            Instruction::HALT => 1,
            Instruction::INT(dval, _rm) => 4 + dval.cycle_cost(flags),
            Instruction::LEA(dval, _rm) => 0 + dval.cycle_cost(flags),
            Instruction::LOAD(dval, _rm) => 2 + dval.cycle_cost(flags),
            Instruction::MUL(dval, _rm) => 20 + dval.cycle_cost(flags),
            Instruction::NEW(dval, _rm) => 16 + dval.cycle_cost(flags),
            Instruction::OPP(dval, _rm) => 2 + dval.cycle_cost(flags),
            Instruction::PEA(dadr) => 4 + dadr.cycle_cost(flags),
            Instruction::POP(_rm) => 2,
            Instruction::PUSH(_rm) => 4,
            Instruction::QUO(dval, _rm) => 40 + dval.cycle_cost(flags),
            Instruction::REM(dval, _rm) => 40 + dval.cycle_cost(flags),
            Instruction::RTS => 8,
            Instruction::SEQ(_rm) => if flags.eq() {3} else {2},
            Instruction::SETROUND_DOWNWARD => 20,
            Instruction::SETROUND_TONEAREST => 20,
            Instruction::SETROUND_TOWARDZERO => 20,
            Instruction::SETROUND_UPWARD => 20,
            Instruction::SGE(_rm) => if flags.ge() {3} else {2},
            Instruction::SGT(_rm) => if flags.gt() {3} else {2},
            Instruction::SHL(_rm) => 2,
            Instruction::SHR(_rm) => 2,
            Instruction::SLE(_rm) => if flags.le() {3} else {2},
            Instruction::SLT(_rm) => if flags.lt() {3} else {2},
            Instruction::SOV(_rm) => if flags.ov() {3} else {2},
            Instruction::SNE(_rm) => if flags.ne() {3} else {2},
            Instruction::STORE(_rm, dadr) => 2 + dadr.cycle_cost(flags),
            Instruction::SUB(dval, _rm) => 2 + dval.cycle_cost(flags),
            Instruction::SUBSP(_imm) => 4,
            Instruction::TSTO(_imm) => 4,
            Instruction::WFLOAT => 16,
            Instruction::WFLOATX => 16,
            Instruction::WINT => 16,
            Instruction::WNL => 14,
            Instruction::WSTR(string) => 16 + string.len() * 2,
            Instruction::WUTF8 => 16,
            Instruction::RINT => 16,
            Instruction::RFLOAT => 16,
            Instruction::RUTF8 => 16,
            Instruction::SCLK => 2,
        }
    }
}