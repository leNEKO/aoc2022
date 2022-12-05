use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Result},
};

pub fn read_lines<T>(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines())
}
