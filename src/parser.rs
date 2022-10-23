
use std::fmt;
use crate::op::Op;
use crate::error::ParseError;

pub struct Parser;


impl Parser {
    pub fn parse( file: String, inpt: String ) -> Result<Vec<Op>, ParseError> {
        let mut line: u32 = 0;
        let mut chr: u32 = 0;
        let words: Vec<String> = Self::chop( inpt );

        let mut r: Vec<Op> = Vec::new();

        for h in words {
        }

        Ok(r)
    }

    fn chop( inpt: String ) -> Vec<String> {
        let split = inpt.split(" ");
        let mut r: Vec<String> = Vec::new();
        for h in split {
            if h.ends_with("\n") {
                let i: usize = h.len() as usize;
                let tmp: &str = &h[..i-1];
                r.push( tmp.trim().to_string() );
                r.push( "\n".to_string() );
                continue;
            }
            r.push( h.trim().to_string() );
        }
        r
    }
}


impl fmt::Display for Parser {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "Parser()" )
    }
}

