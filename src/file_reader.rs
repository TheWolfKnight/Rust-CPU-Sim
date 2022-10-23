
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

pub struct FileReader<'a> {
    path: &'a str,
    completed: bool,
}


impl<'a> FileReader<'a> {
    pub fn new( path: &'a str ) -> Self {
        Self { path, completed: false }
    }

    pub fn read( &self ) -> Result<String, String> {
        let mut r: String = String::new();
        if let Ok(lines) = Self::read_lines(self.path) {
            for line in lines {
                match line {
                    Ok( s ) => r.push_str( s.as_str() ),
                    Err( msg ) => return Err( msg.to_string() ),
                }
            }
        } else {
            return Err( "Could not read the desired file, please check your path!".to_string() );
        }
        Ok(r)
    }

    fn read_lines<P>( path: P ) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

}


impl fmt::Display for FileReader<'_> {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "FileReder( path={}, completed={} )", self.path, self.completed )
    }
}


