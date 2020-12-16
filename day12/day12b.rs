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
    let mut waypoint: (i32,i32) = (10,1);
    let mut heading: char = 'E';
    let i: f64;
    println!("POS: {:?} WAYPOINT: {:?}", pos, waypoint);
    for dir in directions {
        match dir.action {
            'N'|'S'|'E'|'W' => waypoint = mov(waypoint, dir.action,dir.amount),
            'R' => waypoint = rotate(pos,waypoint, -1.0 *dir.amount as f64),
            'L' => waypoint = rotate(pos,waypoint, dir.amount as f64),
            'F' => pos = follow(pos, waypoint, dir.amount),
            _ => ()
        }
        println!("POS: {:?} WAYPOINT: {:?}", pos, waypoint);
    }
    println!("{:?}", pos);
    println!("Distance {:?}", pos.0.abs() + pos.1.abs());
    Ok(())
}

fn mov(waypoint: (i32,i32), heading: char, amount: usize) -> (i32,i32) {
    println!("MOV - HEADING: {} AMOUNT: {}", heading, amount);
    let x = waypoint.0;
    let y = waypoint.1;
    match heading {
        'N' => (x, y + amount as i32),
        'S' => (x, y - amount as i32),
        'E' => (x + amount as i32, y),
        'W' => (x - amount as i32, y),
        _ => (x,y)
    }
    
}

fn follow(pos: (i32,i32), waypoint: (i32,i32), amount: usize) -> (i32,i32) {
    println!("FOL - AMOUNT: {}", amount);
    let xd = waypoint.0 * amount as i32;
    let yd = waypoint.1 * amount as i32;
    return (pos.0+xd, pos.1+yd)
}

fn rotate(pos: (i32,i32), waypoint: (i32,i32), deg: f64) -> (i32,i32) {
    println!("ROT - DEG: {}", deg);
    let rot = deg.to_radians();
    let ox = waypoint.0 ;
    let oy = waypoint.1;
    let xt = ox as f64 * rot.cos() - oy as f64 * rot.sin();
    let yt = ox as f64 * rot.sin() + oy as f64 * rot.cos();
   
    return (xt.round() as i32, yt.round() as i32);
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