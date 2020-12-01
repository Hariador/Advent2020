use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_source_file(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut numbers = vec!();
    for line in reader.lines() {
        let line = line?;
        numbers.push(line.parse().expect("Should be a number"));
        
    }
    Ok(numbers)
}

fn main() {
    let first = env::args().nth(1).expect("Enter input file");
    let numbers = read_source_file(&first).unwrap();

    for i in numbers.iter().enumerate() {
          let subrange = &numbers[i.0..];
          for j in subrange.iter().enumerate() {
              let subrange2 = &subrange[j.0..];
              for k in subrange2 {
                let result = i.1 + j.1 + k;
                match result {
                    2020 => println!("{}",i.1 * j.1 * k),
                    _ => (),
                  };
              }
              
          }
    }
}