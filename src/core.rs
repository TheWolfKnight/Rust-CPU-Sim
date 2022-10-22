
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

    pub fn solve( &mut self ) -> () {
        for (i, op) in self.op_reg.iter().enumerate() {
            match op.get_type() {
                OpType::INA(i) => self.change_reg( *i, 'a' ),
                OpType::INB(i) => self.change_reg( *i, 'b' ),
                OpType::INC(i) => self.change_reg( *i, 'c' ),
                OpType::IND(i) => self.change_reg( *i, 'd' ),
                _ => todo!()
            }
        }
        ()
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


