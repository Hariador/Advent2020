use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::collections::HashSet;

fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter input filename");
    let groups: Vec<HashSet<char>> = read_groups(&filename)?;
    let mut sum = 0;
    for group in groups {
       sum += group.len();
    }
    println!("Total: {}",sum);
    Ok(())
}

fn read_groups(filename: &str) -> io::Result<Vec<HashSet<char>>> {
    let mut groups: Vec<HashSet<char>> = vec!();
  
    let file = File::open(&filename)?;
    let read_buffer = io::BufReader::new(file);
    let mut temp = HashSet::new();
    for line in read_buffer.lines() {
        let line = line?;
        if line == "" {
            groups.push(temp);
            temp = HashSet::new();
        } else {
            for c in line.chars() {
                temp.insert(c);
            }
        }
    }
    if temp.len() > 0 {
        groups.push(temp);
    }

    Ok(groups)
}