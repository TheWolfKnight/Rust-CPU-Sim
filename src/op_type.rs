#![allow(unreachable_patterns)]
#![allow(dead_code)]

use std::fmt;
use crate::core::Regestry;


#[derive(Clone, Copy)]
pub enum OpType {
    NULL,
    SWP,
    CALL,
    PLU(Regestry, Regestry),
    MIN(Regestry, Regestry),
    MLT(Regestry, Regestry),
    DIV(Regestry, Regestry),
    JMP(u64),
    JMPZ(u64),
    IFZ,
    NOT,
    INA(u8),
    INB(u8),
    INC(u8),
    IND(u8)
}


impl fmt::Display for OpType {
    fn fmt(  &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            OpType::NULL                => f.write_str("NULL"),
            OpType::SWP                 => f.write_str("SWP"),
            OpType::CALL                => f.write_str("CALL"),
            OpType::PLU(ref r1, ref r2) => f.write_str("PLU"),
            OpType::MIN(ref r1, ref r2) => f.write_str("MIN"),
            OpType::MLT(ref r1, ref r2) => f.write_str("MLT"),
            OpType::DIV(ref r1, ref r2) => f.write_str("DIV"),
            OpType::JMP(ref i)          => write!( f, "JMP({})",    i),
            OpType::JMPZ(ref i)         => write!( f, "JMPZ({})",   i ),
            OpType::IFZ                 => f.write_str("IFZ"),
            OpType::NOT                 => f.write_str("NOT"),
            OpType::INA(ref i)          => write!( f, "INA({})",    i ),
            OpType::INB(ref i)          => write!( f, "INB({})",    i ),
            OpType::INC(ref i)          => write!( f, "INC({})",    i ),
            OpType::IND(ref i)          => write!( f, "IND({})",    i),
            _ => write!( f, " Please write the last operations in the fmt::Display implementation for OpType ")
        }
    }
}


