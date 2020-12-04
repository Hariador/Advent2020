use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Policy {
    c: char,
    min: usize,
    max: usize
}

fn validate(policy: Policy, password: String) -> bool {
    let mut count = 0;
    let password_chars: Vec<char> = password.chars().collect();
    if password_chars[policy.min] == policy.c && password_chars[policy.max] != policy.c {
        return true;
    }
    if password_chars[policy.min] != policy.c && password_chars[policy.max] == policy.c {
        return true;

    }
    println!("Policy: {:?} Password: {} first: {} second: {}", policy,password, password_chars[policy.min],password_chars[policy.max]);
    false
}

fn read_source_file(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut viable_passwords = 0 ;
    for line in reader.lines() {
        let line = line?;
        let split_line: Vec<&str> = line.split(':').collect();
        let policy_data = split_line[0];
        let password = split_line[1].to_string();
        let policy_data: Vec<&str> = policy_data.split(' ').collect();
        let c: char = policy_data[1].chars().next().unwrap();
        let policy_data: Vec<&str> = policy_data[0].split('-').collect();
        let min: usize = policy_data[0].parse().unwrap();
        let max: usize = policy_data[1].parse().unwrap();
        let policy = Policy {c: c,min: min ,max: max};
        if validate(policy, password)  {
            viable_passwords += 1;
        }
    }

    println!("Viable password count: {}", viable_passwords);
    Ok(())
}

fn main() {
    let first = env::args().nth(1).expect("Enter input file");
    read_source_file(&first).unwrap();

    
}