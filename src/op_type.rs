
use std::fmt;


#[derive(Clone)]
pub enum OpType {
    NULL,
    SWP,
    CAL,
    PLU,
    MIN,
    MLT,
    DIV,
    JMP(u64),
    JMPZ(u64),
    IFZ,
    NOT,
    INA(u8),
    INB(u8),
    INC(u8),
    IND(u8),
}


impl fmt::Display for OpType {
    fn fmt(  &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            OpType::NULL        => f.write_str("NULL"),
            OpType::SWP         => f.write_str("SWP"),
            OpType::CAL         => f.write_str("CAL"),
            OpType::PLU         => f.write_str("PLU"),
            OpType::MIN         => f.write_str("MIN"),
            OpType::MLT         => f.write_str("MLT"),
            OpType::DIV         => f.write_str("DIV"),
            OpType::JMP(ref i)  => write!( f, "JMP({})",    i),
            OpType::JMPZ(ref i) => write!( f, "JMPZ({})",   i ),
            OpType::IFZ         => f.write_str("IFZ"),
            OpType::NOT         => f.write_str("NOT"),
            OpType::INA(ref i)  => write!( f, "INA({})",    i ),
            OpType::INB(ref i)  => write!( f, "INB({})",    i ),
            OpType::INC(ref i)  => write!( f, "INC({})",    i ),
            OpType::IND(ref i)  => write!( f, "IND({})",    i),
            _ => write!( f, " Please write the last operations in the fmt::Display implementation for OpType ")
        }
    }
}


