
use std::fmt;


trait Error {
    fn new( msg: String, file: String, line: ( u32, u32 ) ) -> Self;
}


pub struct ParseError {
    msg: String,
    file: String,
    line: ( u32, u32 )
}


impl Error for ParseError {
    fn new( msg: String, file: String, line: ( u32, u32 ) ) -> Self {
        Self { msg, file, line }
    }
}


impl fmt::Display for ParseError {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}:{}:{} {}", self.file, self.line.0, self.line.1, self.msg )
    }
}
