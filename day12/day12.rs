use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufRead;

#[derive(Debug, Default)]
struct Dir {
    action: char,
    amount: usize,
}

fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter filename");
    let directions: Vec<Dir> = get_data(&filename)?;

    let mut pos: (i32,i32) = (0,0);
    let mut heading: char = 'E';
    for dir in directions {
        match dir.action {
            'N'|'S'|'E'|'W' => pos = mov(dir.action,dir.amount, pos.0, pos.1),
            'R' => {
                match dir.amount {
                    90 => heading = rotate(heading, 1),
                    180 => heading = rotate(heading,2),
                    270 => heading = rotate(heading,3),
                    _ => ()
                }
            },
            'L' => {
                match dir.amount {
                    90 => heading = rotate(heading, -1),
                    180 => heading = rotate(heading,-2),
                    270 => heading = rotate(heading,-3),
                    _ => ()
                }
            },
            'F' => pos = mov(heading,dir.amount, pos.0, pos.1),
            _ => ()
        }
    }
    println!("{:?}", pos);
    println!("Distance {:?}", pos.0.abs() + pos.1.abs());
    Ok(())
}

fn mov(heading: char, amount: usize, x: i32, y: i32) -> (i32,i32) {
    println!("{} {} {} {}", heading, amount, x, y);
    match heading {
        'N' => (x, y + amount as i32),
        'S' => (x, y - amount as i32),
        'E' => (x + amount as i32, y),
        'W' => (x - amount as i32, y),
        _ => (x,y)
    }
}

fn rotate(heading: char, rot: i32) -> char {
    let map: Vec<char> = vec!['E','S','W','N','E','S','W','N','E','S'];
    let mut index: i32 = 0;
    match heading {
        'N' => index = 3,
        'E' => index = 4,
        'S' => index = 5,
        'W' => index = 6,
        _ => ()
    }
    let offset = index + rot;
    return map[offset as usize]
}

fn get_data(filename: &str) -> io::Result<Vec<Dir>> {
    let file = File::open(&filename)?;
    let read_buffer = io::BufReader::new(file);
    let mut directions: Vec<Dir> = vec!();
    for line in read_buffer.lines() {
        let line = line?;
        let input: Vec<char> = line.chars().collect();
        let action = input[0];
        let mut amount: String = String::from("");
        let remainder = input.len();
        for c in 1..remainder {
            amount.push(input[c]);
        }
        let direction = Dir{action: action,amount:amount.parse().unwrap()};
        directions.push(direction);

    }
    Ok(directions)
}