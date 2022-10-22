
use std::fmt;
use crate::op_type::{ OpType };


#[derive(Clone)]
pub struct Op {
    typ: OpType,
}


impl Op {
    pub fn new( typ: OpType ) -> Self {
        Self { typ }
    }

    pub fn get_type( &self ) -> &OpType {
        &self.typ
    }
}

impl fmt::Display for Op {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "Op( type={} )", self.typ )
    }
}

