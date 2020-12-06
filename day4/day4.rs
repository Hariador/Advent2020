use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug, Default)]
struct Passport {
    ecl: String,
    pid: String,
    hcl: String,
    byr: i16,
    iyr: i16,
    eyr: i16,
    cid: i16,
    hgt: String
}

impl Passport {
    fn valid(self) -> bool {
        let ecl: &str = &self.ecl;
        match ecl {
            "amb" |"blu"| "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ => return false
        }

        if self.pid.len() != 9{
            return false;
        }
    
        let split_pid: Vec<char> = self.pid.chars().collect();
        for pid_digit in split_pid {
            match pid_digit {
                '0'..='9' => (),
                _ => return false
            }
        }
      
        if self.hcl.len() != 7 {
            return false;
        }

        let mut split_hcl: Vec<char> = self.hcl.chars().collect();
        if split_hcl[0] != '#' {
            return false;
        }
        split_hcl.remove(0);

        for hcl_digit in split_hcl {
            match hcl_digit {
                '0'..='9' => (),
                'a'..='f' => (),
                _ => return false
            }
        }
        let hgt_unit = self.hgt.trim_matches(char::is_numeric); 
        
        let mut hgt_value: i32 = 0; 
        match self.hgt.trim_matches(char::is_alphabetic).parse() {
            Ok(x) => hgt_value = x,
            _ => return false
        }
        match hgt_unit {
            "cm" => {
                if hgt_value < 150 || hgt_value > 193 {
                    return false;
                }
            },
            "in" => {
                if hgt_value < 59 || hgt_value > 76 {
                    return false;
                }
            },
            _ => return false
        }


        if self.byr < 1920 || self.byr > 2002 {
            return false;
        }

        if self.iyr < 2010 || self.iyr > 2020 {
            return false;
        }

        if self.eyr < 2020 || self.eyr > 2030 {
            return false;
        }

        true
    }

    pub fn new() -> Passport {
        Passport{
         ecl:"".to_string(),
         pid:"".to_string(),
         hcl:"".to_string(),
         byr:0,iyr:0,cid:0,eyr:0,
         hgt:"".to_string()}
    }


}

fn read_chunk(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut parsed_passports = vec!();
    let mut temp: String = String::from("");
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            parsed_passports.push(temp); 
            temp = String::from("");
        } else { 
            temp += " ";
            temp += &line;
        }      
    }
    if temp.len() != 0 {
        parsed_passports.push(temp);
    }
    Ok(parsed_passports)
}

fn parse_data(raw_data: &Vec<String>) -> Vec<Passport> {
    let mut parsed_passports: Vec<Passport> = vec!();
    for data in raw_data {
        let mut p: Passport = Passport::new();
        for word in data.split(' ') {
            let split_word: Vec<&str> = word.split(':').collect();
            match split_word[0] {
                "ecl" => p.ecl = String::from(split_word[1]),
                "hcl" => p.hcl = String::from(split_word[1]),
                "hgt" => p.hgt = String::from(split_word[1]),
                "pid" => p.pid = String::from(split_word[1]),
                "byr" => p.byr = split_word[1].parse().unwrap(),
                "iyr" => p.iyr = split_word[1].parse().unwrap(),
                "eyr" => p.eyr = split_word[1].parse().unwrap(),
                "cid" => p.cid = split_word[1].parse().unwrap(),
                _ => ()
            }
        }
        parsed_passports.push(p);
    }

    return parsed_passports;
}

fn main() -> Result<(), io::Error> {

    let first = env::args().nth(1).expect("Enter input file");
    let passport_data = read_chunk(&first)?;
    let parsed_passports = parse_data(&passport_data);
    let mut valid = 0;
    for passport in parsed_passports {
       
        if passport.valid() {
            valid += 1;
           
        } else {
            //println!("INVALID");
        }
    }
    println!("There were {} valid passports", valid);

    Ok(())
}