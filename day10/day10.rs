use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufRead;
use std::collections::HashMap;


fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter a filename");
    let mut adapters: Vec<usize> = read_data(&filename)?;
    let mut cache: HashMap<usize,usize> = HashMap::new();
    adapters.sort();
    println!("Data: {:?}", adapters);
    println!("Total solutions is {}",solve(&mut adapters));
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

fn solve(adapters: &mut Vec<usize>) -> usize {
    let mut max = adapters.len()-1;
    let mut i: usize = 1;
    let mut muli: usize = 1;
    loop {
       match find_anchor(&adapters) {
           Some(i) => {
               let mut chunk = adapters.split_off(i);
               
               let c = count_solutions(&mut chunk,1);
               println!("For {:?} count {}", chunk,c);
               muli = c * muli;
           },
           None => {
            let c = count_solutions(adapters,1);
            println!("For {:?} count {}", adapters,c);
            muli = c * muli;
            break;
            }
       }
    }

    return muli
}

fn find_anchor(adapters: &Vec<usize>) -> Option<usize> {
    let mut end = adapters.len()-1;
    
    for i in 1..end {
        let tar = end -i;
        if adapters[tar] - adapters[tar-1] == 3 {
            return Some(tar);
        }
    }
    
    return None
}


fn count_solutions(adapters: &mut Vec<usize>, tail: usize) -> usize {
   let mut sol: usize = 0;
   let max: usize = adapters.len()-1;
   if valid(adapters) {
       println!("{:?}", adapters);
       sol += 1;
       for i in tail..max {
            let tmp = adapters[i];
            adapters.remove(i);
            sol += count_solutions(adapters, i);
            adapters.insert(i,tmp);
       }
   } 

   return sol;
    
}

fn valid(adapters: &Vec<usize>) -> bool {
    
    let mut last: usize = adapters[0];

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
