
use std::fmt;
use crate::op_type::{ OpType };


#[derive(Clone, Copy)]
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


pub struct OpVector {
    pub content: Vec<Op>,
    pub file: String,
}


impl OpVector {
    pub fn new( content: Vec<Op>, file: String ) -> Self {
        Self { content, file }
    }

}

