use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufRead;
use std::collections::HashMap;

fn main() -> Result<(),io::Error> {
   let filename = env::args().nth(1).expect("Filename");
   let sum = process_inputs(&filename)?;
    println!("SUM {}", sum );
   Ok(())
}

fn process_inputs(filename: &str) -> io::Result<i64> {

    let mut registers: HashMap<i64,i64> = HashMap::new();
    let mut mask: Vec<char> = vec!();
    let mut sum = 0_i64;
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let split_line: Vec<&str> = line.split('=').collect();
        match split_line[0] {
            "mask " => mask = split_line[1].chars().collect(),
            a => {
                let value = split_line[1].trim_left().parse::<i64>().unwrap();
                let address = parse_address(a);
                let addresses = mask_value(address, &mask);
                for address in addresses {
                    registers.insert(address, value);
                }
            },
        }
       
    }
    println!("Registers {:?}", registers);
    for (_,value) in &registers {
        sum += value;
    }

    Ok(sum)
}

fn parse_address(address_data: &str) -> i64 {
    
    let v = ['[',']'] ;
    let split_address: Vec<&str> = address_data.split(|c| c == ']' || c == '[').collect();
  
    let address = split_address[1].parse::<i64>().unwrap();

    return address
}

fn mask_value(v: i64, mask: &Vec<char>) -> Vec<i64> {
    let mut bitmask = 1_i64 << 36;
    let mut value = v;
    let mut addresses: Vec<i64> = vec!();
    addresses.push(0);
    let mut swap: Vec<i64> = vec!();
    for bit in mask {
        swap = vec!();
        match bit {
            'X' => {
                for address in addresses {
                    let a = address << 1;
                    swap.push(a + 1);
                    swap.push(a + 0);
                }
                addresses = swap;
            },
            '1' => {
                let b = 1;
                for address in addresses {
                    let a = address << 1;
                    swap.push(a + b);
                }
                addresses = swap;
            }
            '0' => {
                let mut b = (value & bitmask);
                if b > 0 {
                    b = 1 ;
                }
                for address in addresses {
                    let a = address << 1;
                    swap.push(a + b);
                }
                addresses = swap;
            }
            
            _ =>  ()
        }
        bitmask = bitmask >> 1;
     
    }

    return addresses
}



