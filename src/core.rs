
use crate::error::CoreSolveError;
use crate::op::{ Op, OpVector };
use crate::op_type::OpType;


#[ derive(Clone, Copy) ]
pub enum Regestry {
    Data(u8),
    Pointer(u32),
    NULL,
}


impl Regestry {
    pub fn type_to_string( &self ) -> String {
        match self {
            Self::Data(_) => return "Data".to_string(),
            Self::Pointer(_) => return "Pointer".to_string(),
            Self::NULL => return "NULL".to_string(),
        }
    }
}


pub struct Core {
    reg_a: Regestry,
    reg_b: Regestry,
    reg_c: Regestry,
    reg_d: Regestry,
    op_reg: OpVector,
    file: String,
}


impl Core {
    pub fn new( op_reg: OpVector, file: String ) -> Self {
        Self { reg_a: Regestry::NULL, reg_b: Regestry::NULL, reg_c: Regestry::NULL, reg_d: Regestry::NULL, op_reg, file }
    }

    pub fn solve( &mut self ) -> Result<(), CoreSolveError> {
        let mut i: usize = 0;
        while i < self.op_reg.content.len() {
            let op: Op = self.op_reg.content[i];
            match op.get_type() {
                OpType::INA(i) => self.change_reg( *i, 'a' ),
                OpType::INB(i) => self.change_reg( *i, 'b' ),
                OpType::INC(i) => self.change_reg( *i, 'c' ),
                OpType::IND(i) => self.change_reg( *i, 'd' ),
                OpType::NULL => {},
                OpType::SWP => {
                    let tmp_a: Result<Regestry, ()> = self.get_reg_consumed( 'a' );
                    match tmp_a {
                        Ok(i) => {
                            self.reg_a = self.reg_b;
                            self.reg_b = i;
                        }
                        Err(()) => {
                            return Err( CoreSolveError::new( "Cannot swap an empty a registor into the b registor".to_string(), (*self.file).to_string(), ( i, 0 )) );
                        }
                    }
                },
                OpType::CALL        => {
                    if let Regestry::Data(reg_a_data) = self.reg_a {
                    } else {
                        return Err( CoreSolveError::new("An empty a regestry cannot be interpreted as a valid syscall!".to_string(), (*self.file).to_string(), ( i, 0 )) );
                    }
                },
                OpType::PLU(r1, r2) => {},
                OpType::MIN(r1, r2) => {},
                OpType::MLT(r1, r2) => {},
                OpType::DIV(r1, r2) => {},
                OpType::JMP(i)      => {},
                OpType::JMPZ(i)     => {},
                OpType::IFZ         => {},
                OpType::NOT         => {},
                _ => todo!()
            }
            i += 1;
        }
        Ok(())
    }

    fn get_reg_consumed( &self, reg_key: char ) -> Result<Regestry, ()> {
        if reg_key == 'a' {
            return Ok(self.reg_a.clone());
        } else if reg_key == 'b' {
            return Ok(self.reg_b.clone());
        } else if reg_key == 'c' {
            return Ok(self.reg_c.clone());
        } else if reg_key == 'd' {
            return Ok(self.reg_d.clone());
        }
        Err(())
    }

    fn change_reg( &mut self, inpt: u8, reg_key: char ) -> () {
        if reg_key == 'a' {
            self.reg_a = Regestry::Data(inpt);
        } else if reg_key == 'b' {
            self.reg_b = Regestry::Data(inpt);
        } else if reg_key == 'c' {
            self.reg_c = Regestry::Data(inpt);
        } else if reg_key == 'd' {
            self.reg_d = Regestry::Data(inpt);
        }
    }

}


