use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_source_file(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split(',').collect();
        println!("{:?}",numbers);
    }
    Ok(())
}

fn main() {
    let file = env::args().nth(1).expect("Filename required");
    read_source_file(&file).expect("Could not read file");
   
}