/// Created by Virgile HENRY, 2023/09/28

use crate::ima::{data_type::DataType, address_modes::RegisterIndex};

/// The Register set of the machine.
/// This only contains the registers from 0 to 15, not LB, GB and SP.
#[cfg(not(feature = "public-ima"))]
pub struct Registers {
    registers: Vec<DataType>,
}

#[cfg(feature = "public-ima")]
pub struct Registers {
    pub registers: Vec<DataType>,
}


impl Registers {
    /// Creates a new register set with the given number of registers.
    pub fn new(r_count: usize) -> Registers {
        Registers {
            registers: vec![DataType::Undefined; r_count],
        }
    }

    /// Get the data type in the given register.
    pub fn get(&self, index: RegisterIndex) -> DataType {
        self.registers[usize::from(index.0)]
    }

    /// Set the data type in the given register.
    pub fn set(&mut self, index: RegisterIndex, value: DataType) {
        self.registers[usize::from(index.0)] = value;
    }

    pub fn display(&self, output: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut new_line = false;
        for (i, r) in self.registers.iter().enumerate() {
            match new_line {
                true => writeln!(output, "{:>15}R{:<2} : {}", ' ', i, r)?,
                false => write!(output, "R{:<2} : {}", i, r)?,
            }
            new_line = !new_line;
        }
        Ok(())
    }
}