use std::cmp;
use std::env;

fn main() {
    println!("TEST STUFF");
    let row_max = 10;
    let seat_max = 10;
    let row = env::args().nth(1).expect("Enter a number").parse::<u32>().expect("Enter a number");
    let seat = env::args().nth(2).expect("Enter a number").parse::<u32>().expect("Enter a number");
   
    println!("NORTH");
    
    for i in 1..row+1 {
        println!("{} {}", row-i,seat);
    }

    
    //SE Diagonal
    println!("SE");
    let range = cmp::min((seat_max-1 - seat), (row_max -1 - row));
    for i in 1..=range {
        println!("{} {}", row+i,seat+i);
    }
    println!("SW");
    let range = cmp::min(seat, (row_max -1 - row));
    for i in 1..=range {
        println!("{} {}", row+i,seat-i);
    }
    println!("NW");
    let range = cmp::min(row,seat);
    for i in 1..=range {
        println!("{} {}", row-i,seat-i);
    }
    println!("NE");
    let range = cmp::min(row,(seat_max-1 - seat));
    for i in 1..=range {
        println!("{} {}", row-i,seat+i);
    }
}