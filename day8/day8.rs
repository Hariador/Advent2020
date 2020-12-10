use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::collections::HashMap;


#[derive(Debug, Default)]
struct Instruction {
    ins: String,
    val: i32,
    has_run: bool
}

impl Instruction {
    fn run(&mut self) {
        self.has_run = true;
    }

    fn reset(&mut self) {
        self.has_run = false;
    }

    fn swap(&mut self) {
        match self.ins.as_str() {
            "nop" => self.ins = "jmp".to_string(),
            "jmp" => self.ins = "nop".to_string(),
            _ => ()
        }
    }
}

fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter filename");
    let mut program = build_program(&filename)?;
    let length: i32 = program.len() as i32;
    println!("OP COUNT: {}", length);

    for i in 0..length{
        let mut instruction = program.get_mut(&i).unwrap();
  
        instruction.swap();

        let instruction = ();
       
        match does_terminate(&mut program) {
            Some(acc) => println!("TERM: {}", acc),
            None => ()
        }
        let mut instruction = program.get_mut(&i).unwrap();
        
        instruction.swap();
  
        reset(&mut program);
    }
   
    Ok(())
}

fn reset(program: &mut HashMap<i32,Instruction>) {
    for (_,i) in program {
        i.reset(); 
    }
}


fn does_terminate(program: &mut HashMap<i32,Instruction>) -> Option<i32> {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let length = program.len() as i32;
    loop {
        let mut instruction = program.get_mut(&pc)?;
    
        if instruction.has_run {
            return None ;
        }
        
     
        match instruction.ins.as_str() {
            "nop" => {
                instruction.run();
                pc += 1;
            },
            "acc" => {
                acc += instruction.val;
                instruction.run();
                pc += 1;
            },
            "jmp" => {
                instruction.run();
                pc += instruction.val;
            },
            _ => ()
        }
        
        if pc >= length {
            return Some(acc);
        }
        
    }

}

fn build_program(filename: &str) -> io::Result<HashMap<i32,Instruction>> {
    let mut program: HashMap<i32,Instruction> = HashMap::new();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut address: i32 = 0;
    for line in reader.lines() {
        
        let line = line?;
        let temp: Vec<&str> = line.split(' ').collect();
        let i = temp[0];
        let mut value = temp[1].to_string();
        let sign = value.remove(0);
        let mut v : i32 = 0;
        match sign {
            '+' =>  v = value.parse::<i32>().unwrap(),
            '-' =>  v = -1 * value.parse::<i32>().unwrap(),
            _ => ()
        }
        
     
        let instruction = Instruction{ins:i.to_string(),val:v,has_run:false};
        program.insert(address, instruction);
        address += 1;
    }

    Ok(program)
}