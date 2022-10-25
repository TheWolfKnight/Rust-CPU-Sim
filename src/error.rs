
use std::fmt;

pub struct ParseError {
    msg: String,
    file: String,
    line: ( usize, usize )
}


impl ParseError {
    pub fn new( msg: String, file: String, line: ( usize, usize ) ) -> Self {
        Self { msg, file, line }
    }
}


impl fmt::Display for ParseError {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}:{}:{} {}", self.file, self.line.0, self.line.1, self.msg )
    }
}


pub struct CoreSolveError {
    msg: String,
    file: String,
    op_index: usize,
}


impl CoreSolveError {
    pub fn new( msg: String, file: String, line: ( usize, usize ) ) -> Self {
        Self { msg, file, op_index: line.0 }
    }
}


impl fmt::Display for CoreSolveError {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}:{}:0: {}", self.file, self.op_index, self.msg)
    }
}

