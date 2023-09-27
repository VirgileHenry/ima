/// Created by Virgile HENRY, 2023/09/28


use super::memory::Pointer;


/// All flags the ima machine can have.
pub struct Flags {
    /// Equality
    eq: bool,
    /// Not equality
    ne: bool,
    /// Greater than
    gt: bool,
    /// Greater than or equality
    ge: bool,
    /// Less than
    lt: bool,
    /// Less than or equality
    le: bool,
    /// Overflow
    ov: bool,
}

impl Flags {
    /// Create a new flag array, with the default values.
    /// The flags set to true by default are NE, GT, GE.
    pub fn new() -> Flags {
        Flags {
            eq: false,
            ne: true,
            gt: true,
            ge: true,
            lt: false,
            le: false,
            ov: false,
        }
    }

    /// Fetch the value of the EQ flag.
    pub fn eq(&self) -> bool { self.eq }
    /// Fetch the value of the NE flag.
    pub fn ne(&self) -> bool { self.ne }
    /// Fetch the value of the GT flag.
    pub fn gt(&self) -> bool { self.gt }
    /// Fetch the value of the GE flag.
    pub fn ge(&self) -> bool { self.ge }
    /// Fetch the value of the LT flag.
    pub fn lt(&self) -> bool { self.lt }
    /// Fetch the value of the LE flag.
    pub fn le(&self) -> bool { self.le }
    /// Fetch the value of the OV flag.
    pub fn ov(&self) -> bool { self.ov }

    /// Set the value of the OV flag.
    pub fn set_ov(&mut self, value: bool) { self.ov = value; }

    /// Set all comparaison flags based on the two compare ints values.
    /// This is done how the CMP instruction would do it.
    pub fn set_cmp_int(&mut self, dval: i32, rm: i32) {
        self.eq = rm == dval;
        self.ne = !self.eq;
        self.lt = rm < dval;
        self.le = self.lt || self.eq;
        self.gt = !self.le;
        self.ge = !self.lt;
    }

    /// Set all comparaison flags based on the two compare floats values.
    /// This is done how the CMP instruction would do it.
    pub fn set_cmp_float(&mut self, dval: f32, rm: f32) {
        self.eq = rm == dval;
        self.ne = !self.eq;
        self.lt = rm < dval;
        self.le = self.lt && self.ne;
        self.gt = !self.le;
        self.ge = !self.lt;
    }

    /// Set all comparaison flags based on the two compare pointers values.
    /// This is done how the CMP instruction would do it.
    pub fn set_cmp_ptr(&mut self, dval: Pointer, rm: Pointer) {
        self.eq = rm == dval;
        self.ne = !self.eq;
        self.lt = self.ne; // convention
        self.le = self.lt && self.ne;
        self.gt = !self.le;
        self.ge = !self.lt;
    }
    
    pub fn display(&self, output: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        write!(output, "Flags set to true: ")?;
        if self.eq { write!(output, "EQ ")?; }
        if self.ne { write!(output, "NE ")?; }
        if self.gt { write!(output, "GT ")?; }
        if self.ge { write!(output, "GE ")?; }
        if self.lt { write!(output, "LT ")?; }
        if self.le { write!(output, "LE ")?; }
        if self.ov { write!(output, "OV ")?; }
        writeln!(output)?;
        Ok(())
    }
}