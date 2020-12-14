use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufRead;
use std::collections::HashMap;


fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter a filename");
    let mut adapaters: Vec<usize> = read_data(&filename)?;
    let mut cache: HashMap<usize,usize> = HashMap::new();
    adapaters.sort();
    adapaters.push(adapaters.last().unwrap() + 3);
    let solutions = solve(&mut adapaters, &mut cache, 0);
    println!("Solutions: {}", solutions);
    Ok(())

}

fn read_data(filename: &str) -> io::Result<Vec<usize>> {
    let mut input_data: Vec<usize> = vec!();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    input_data.push(0);
    for line in reader.lines() {
        let line = line?;
        let x: usize = line.parse().unwrap();
        input_data.push(x);
    }

    Ok(input_data)
}

fn solve(adapters: &mut Vec<usize>, cache: &mut HashMap<usize, usize>, tail: usize) -> usize {
    
    let mut solutions: usize = 0;
  //  println!("{:?} TAIL {}",adapters, tail);
    if valid(adapters) {
        solutions += 1;
        for i in tail..adapters.len()-2 {
            let tmp = adapters[i+1];
           
            match cache.get(&tmp) {
                Some(x) =>  {
                  //  println!("{:?} CACHE HIT FOR {} VALUE {}",adapters, tmp, x);
                    if adapters[i+2] - adapters[i] <= 3 {
                       
                        adapters.remove(i+1);
                        solutions += solve(adapters, cache, i);
                      
                        cache.insert(tmp,solutions);
                   
                        adapters.insert(i+1, tmp);
                        println!("Solving for {} is {}", tmp, solutions);
                    }
                },
                None => {
                    if adapters[i+2] - adapters[i] <= 3 {
                       
                        adapters.remove(i+1);
                        solutions += solve(adapters, cache, i);
                      
                        cache.insert(tmp,solutions);
                   
                        adapters.insert(i+1, tmp);
                        println!("Solving for {} is {}", tmp, solutions);
                    }
                }
            }  
        }
    }

    return solutions;
}



fn valid(adapters: &Vec<usize>) -> bool {
    
    let mut last: usize = 0;
    for i in adapters {
        let diff = i - last;
   
        match diff {
            0|1|2|3 => (),
            _ => return false
        }
        last = *i;
    }

    return true
}
