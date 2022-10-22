
mod op_type;
mod op;
mod core;

use crate::op_type::OpType;
use crate::op::Op;
use crate::core::Core;


fn main() {

    let mut vec_of_ops: Vec<Op> = Vec::new();
    vec_of_ops.push( Op::new( OpType::INA(6) ) );
    vec_of_ops.push( Op::new( OpType::INB(7) ) );
    vec_of_ops.push( Op::new( OpType::SWP ) );

    let mut core: Core = Core::new( vec_of_ops );
    core.solve();

}

