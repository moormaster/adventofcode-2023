pub mod input_helper {
    use std::fs::File;
    use std::io::{Lines, Result, BufReader};
    use std::io::prelude::*;

    pub fn read_lines(file: &str) -> Result<Lines<impl BufRead>> {
        let input_file = File::open(file)?;
        let reader = BufReader::new(input_file);
        
        Ok(reader.lines())
    }
}
