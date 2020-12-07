use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::collections::HashMap;

fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter input filename");
    let sum = read_groups(&filename)?;
    
    println!("Total: {}",sum);
    Ok(())
}

fn read_groups(filename: &str) -> io::Result<i32> {
    let file = File::open(&filename)?;
    let read_buffer = io::BufReader::new(file);
    let mut temp = HashMap::new();
    let mut group_size = 0;
    let mut sum = 0;
    for line in read_buffer.lines() {
        let line = line?;
        if line == "" {
            sum += count_group(&temp, group_size);
            temp.clear();
            group_size = 0;
        } else {
            group_size += 1;
            for ch in line.chars() {
                let count = temp.entry(ch).or_insert(0);
                *count += 1;
            }
        }
    }
    sum += count_group(&temp, group_size);

    Ok(sum)
}

fn count_group(group: &HashMap<char,i32>, group_size: i32) -> i32 {
    let mut sum = 0;
    for (_,count) in group {
        if count == &group_size {
            sum += 1;
        }
    }
    println!("{}",sum);
    return sum;
}