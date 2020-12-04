use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() {

    let filename = env::args().nth(1).expect("Enter input file");
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut width = 0;
    let mut height = 0;
    let mut row_buffer: Vec<Vec<char>> = vec!();
    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        if width == 0 {
            width = chars.len();
        }
        height += 1;
        row_buffer.push(chars);
    }
    println!("Width: {} Height: {}", width,height);
    let mut trees = 0;
    for i in 0..height {
        let x = (3 * i) % width;
        println!("X:{} Y:{}", x, i);
    }

    
}
