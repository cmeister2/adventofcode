use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};

pub fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.map(|line| line.unwrap()).collect())
}
