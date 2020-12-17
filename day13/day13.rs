use std::io;
use std::fs::File;
use std::env;
use std::io::BufRead;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};


fn main() {
    let filename = env::args().nth(1).expect("Enter Filename");
    let (start_time, busses) = get_data(&filename);
    let mut tick: i64 = 0;

    println!("StartTime: {}", start_time);
    println!("Busses: {:?}", busses);
    let (max_bus,offset) = busses.iter().last().unwrap();
    let bus_count = busses.len();
    let mut sum: i64 = 0;
    let n = calc_big_n(&busses);
    println!("BIG N: {}", n);
    for (bus, second) in busses.iter() {
        let p = n / bus ;
        sum += (bus - second) * p * mod_inv(p, *bus)
    }
    println!("TICK: {}", sum % n);
}

fn egcd(y: i64, base: i64 ) -> (i64, i64, i64) {
    if y == 0 {
        (base, 0, 1)
    } else {
        let (gcd, bc_a, bc_b) = egcd(base % y, y);
        (gcd, bc_b - ( base / y) * bc_a, bc_a)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (gcd, x, _) = egcd(x,n);
    return (x % n + n) % n
}

fn calc_big_n(busses: &BTreeMap<i64,i64>) -> i64 {
    let mut product: i64 =1 ;
    for (bus,_) in busses.iter() {
        product = product * bus;
    }

    return product
}

fn get_data(filename: &str) -> (i64,BTreeMap<i64,i64>) {

    let mut starttime: i64 = 0;
    let mut busses: BTreeMap<i64,i64> = BTreeMap::new();
    match File::open(&filename) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let mut read_iter = reader.lines();
         
            match read_iter.next() {
                Some(x) => starttime = x.unwrap().parse::<i64>().unwrap(),
                None => ()
            }
            let mut line = String::from("");
            match read_iter.next() {
                Some(x) => line = x.unwrap(),
                None => ()
            }
            let mut pos: i64 = 0;
            for bus in line.split(',') {
                match bus {
                    "x" => pos += 1,
                    _ => {
                        busses.insert(bus.parse::<i64>().unwrap(),pos);
                        pos += 1;
                    }
                }
            }
        },
        Err(_) => () 
    }

    return (starttime, busses)
}