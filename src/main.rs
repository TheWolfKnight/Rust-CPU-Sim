#![allow(unused_variables)]

mod op_type;
mod op;
mod core;
mod file_reader;
mod error;
mod parser;

use crate::op_type::OpType;
use crate::op::Op;
use crate::core::Core;
use crate::file_reader::FileReader;
use crate::parser::Parser;


fn main() {

    let mut vec_of_ops: Vec<Op> = Vec::new();
    vec_of_ops.push( Op::new( OpType::INA(6) ) );
    vec_of_ops.push( Op::new( OpType::INB(7) ) );
    vec_of_ops.push( Op::new( OpType::SWP ) );

    let mut core: Core = Core::new( vec_of_ops );
    core.solve();
}

