use std::io;
use std::io::Error;
use std::env;
use std::io::BufRead;
use std::fs::File;

fn main() -> Result<(), io::Error> {
    let filename = env::args().nth(1).expect("Enter Filename");
    let boardingpasses = read_pass_data(&filename)?;
    let mut seats: Vec<i32> = vec!();
    for pass in boardingpasses {
       seats.push(find_id(&pass));
    }
    let mut inc = 28;
    seats.sort();
    for id in seats {
        if inc != id {
            println!("{}",inc);
        } else {
            inc += 1;
        }
    }
    Ok(())
}

fn read_pass_data(filename: &str) -> io::Result<Vec<String>> {
    let mut data: Vec<String> = vec!();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}

fn find_id(row_code: &str) -> i32 {
    let mut lower = 0;
    let mut upper = 128;
    let mut lower_seat = 0;
    let mut upper_seat = 8;
    
    for step in row_code.chars() {
        match step {
            'F' => {
                let mid = (upper - lower) / 2;
                upper -= mid;
               
            },
            'B' => {
                let mid = (upper - lower) / 2;
                lower += mid;
             
            },
            'L' => {
                let mid = (upper_seat - lower_seat) / 2;
                upper_seat -= mid;
               
            },
            'R' => {
                let mid = (upper_seat - lower_seat) / 2;
                lower_seat += mid;
             
            },
            _ => ()
        }
    }
    upper -= 1;
    upper_seat -=1;
    let id = (upper * 8) + upper_seat;
    return id;
}