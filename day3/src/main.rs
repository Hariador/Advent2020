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
        println!("{:?}",chars);
        row_buffer.push(chars);
       
    }
    println!("Width: {} Height: {}", width,height);
    let mut trees = 0;
    for y in 0..height {
        for x in 0..width {
            print!("{}",row_buffer[y][x]);
        }
        println!();
    }
    let slopes = vec!((1,1),(3,1),(5,1),(7,1),(1,2));
    let mut total: u64  = 1;
    for slope in slopes {
        trees = 0;
        for i in 0..height {
           
            let x = (slope.0 * i) % width;
            let y = slope.1 * i ;
            if y < height {
                if row_buffer[y][x] == '#' {
                
                    trees += 1;
                } else {
                
                    //println!("{},{}",x,i);
                }
            }
        }
        println!("Hit {} trees", trees);
        total *= trees;

    }
    println!("T Total: {}",total);


    
}
