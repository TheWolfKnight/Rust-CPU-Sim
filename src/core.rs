
use crate::op::{ Op };
use crate::op_type::{ OpType };

pub struct Core {
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    op_reg: Vec<Op>,
}


impl Core {
    pub fn new( op_reg: Vec<Op> ) -> Self {
        Self { reg_a: 0, reg_b: 0, reg_c: 0, reg_d: 0, op_reg }
    }

    pub fn solve( &mut self ) -> Result<(), String> {
        for i in 0..self.op_reg.len() {
            let op: Op = self.op_reg[i];
            match op.get_type() {
                OpType::INA(i) => self.change_reg( *i, 'a' ),
                OpType::INB(i) => self.change_reg( *i, 'b' ),
                OpType::INC(i) => self.change_reg( *i, 'c' ),
                OpType::IND(i) => self.change_reg( *i, 'd' ),
                OpType::NULL => {},
                OpType::SWP => {
                    let tmp_a: Result<u8, ()> = self.get_reg( 'a' );
                    match tmp_a {
                        Ok(i) => {
                            self.reg_a = self.reg_b;
                            self.reg_b = i;
                        }
                        Err(()) => {
                            return Err( "Cannot swap an empty a registor into the b registor".to_string() );
                        }
                    }
                }
                _ => todo!()
            }
        }

        Ok(())

    }

    pub fn get_reg( &self, reg_key: char ) -> Result<u8, ()> {
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
            self.reg_a = inpt;
        } else if reg_key == 'b' {
            self.reg_b = inpt;
        } else if reg_key == 'c' {
            self.reg_c = inpt;
        } else if reg_key == 'd' {
            self.reg_d = inpt;
        }
    }

}


