use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;


fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter filename");
    let preamble_size: usize = env::args().nth(2)
        .expect("Enter preamble size")
        .parse()
        .expect("Enter preample size as integer");
    let data_stream: Vec<usize> = read_input(&filename)?;
    let data_size:usize = data_stream.len();
    println!("SUM: {}", sum(&data_stream[..]));
    for pointer in 0..data_size-preamble_size {
        
        match find_in_previous(&data_stream[pointer..pointer+preamble_size], data_stream[pointer+preamble_size]) {
            Some(value) => {
                for i in 0..data_size {
                    for j in i..data_size {
                        if sum(&data_stream[i..j]) == value {
                            println!("{:?}", &data_stream[i..j]);
                            let (min,max) = get_min_max(&data_stream[i..j]);
                            println!("MIN : {} MAX: {} SUM: {}", min, max, min+ max);
                            break;
                        }
                    }
                }
            },
            None => (),
        }
    }
 
    Ok(())
}

fn get_min_max(data: &[usize]) -> (usize,usize) {
    let mut min: usize = std::usize::MAX;
    let mut max: usize = 0;
    for i in data {
        if i < &min {
            min = *i;
        }
        if i > &max {
            max = *i;
        }
    }
    return (min,max)
}

fn sum(list: &[usize]) -> usize {
    let mut total: usize = 0;
    for i in list {
        total += i;
    }

    return total;
}

fn find_in_previous(previous: &[usize],value: usize) -> Option<usize>{
    let mut index: usize = 0;
    let size: usize = previous.len();

    for i in previous {
        index += 1;
        for j in previous[index..size].into_iter() {
           
            if i + j == value {
                return None;
            }
        }
    }
    return Some(value)
}
fn read_input(filename: &str) -> io::Result<Vec<usize>> {
    let mut data: Vec<usize> = vec!();
    let file = File::open(&filename)?;
    let read_buffer = io::BufReader::new(file);
    for line in read_buffer.lines() {
        let line = line?;
        data.push(line.parse().unwrap());
    }

    Ok(data)

}