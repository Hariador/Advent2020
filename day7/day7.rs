use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), io::Error> {
    let filename = env::args().nth(1).expect("Filename");
    let rules = get_rules(&filename)?;
    let paths = find(rules);
    println!("Answer: {}", paths);
    Ok(())
}

fn get_rules(filename: &str) -> io::Result<HashMap<String,HashSet<String>>> {
    let file = File::open(&filename)?;
    let input_buffer = io::BufReader::new(file);
    let mut rules: HashMap<String,HashSet<String>> = HashMap::new();
    let mut total = 0;
    for  line in input_buffer.lines() {
        let line = line?;
        let mut split_lin_iter = line.split(" contain ");
        let key_data = split_lin_iter.next().unwrap();
        let rule_data = split_lin_iter.next().unwrap();
        rules.insert(get_rule_key(&key_data),get_rule_set(&rule_data));
        total += 1;
    }
    println!("Total of {} rules", total);
    Ok(rules)
}

fn get_rule_set(raw_rule: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    
    if raw_rule == "no other bags." {
        return set;
    }
    for rule in raw_rule.split(',') {
       
        let temp: Vec<&str> = rule.trim().split(char::is_whitespace).collect();
        let key: String = String::from(temp[1]) + temp[2];
        set.insert(key);
    }
    set
}

fn get_rule_key(rule: &str) -> String {
    let temp: Vec<&str> = rule.trim().split(char::is_whitespace).collect();
    let key: String = String::from(temp[0]) + temp[1];

    return key
}

fn find(working_set: HashMap<String,HashSet<String>> ) -> i32 {
    let mut search_targets: HashSet<String> = HashSet::new();
    let mut temp: HashSet<String> = HashSet::new();
    let mut rules = working_set;
    search_targets.insert("shinygold".to_string());
    let mut total = 0;
    let mut debug_counter = 0;
    loop {
        debug_counter += 1;
        for (k, rule) in &rules {
            for search in &search_targets {
                if rule.contains(search) {
                    temp.insert(k.to_string());
                }
            }
        }
        total += temp.len()as i32;
        if temp.len() == 0 {
            break;
        }
        search_targets.clear();
        for key in &temp {
            search_targets.insert(key.to_string());
            rules.remove_entry(key);
        }
  
        temp.clear();
    }

    println!("Loops {}", debug_counter);
    return total;
}

//light red bags contain 1 bright white bag, 2 muted yellow bags.
//io::Result<Vec<HashMap<String,HashSet<String>>>>
