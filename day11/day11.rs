use std::io;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufRead;
use std::cmp;


fn main() -> Result<(),io::Error> {
    let filename = env::args().nth(1).expect("Enter filename");
    let (mut seats, seat_max, row_max) = read_data(&filename);
    
    let mut tmp: Vec<Vec<char>> = get_blank(seat_max, row_max);
    let mut skip = false;
    println!("ROWS: {} SEATS: {}", row_max, seat_max);
    loop {
        let mut changed = false;
        for row in 0..row_max {
            for seat in 0..seat_max {
                match seats[row][seat] {
                    '.' => tmp[row][seat] = '.',
                    'L' => {
                        match adj_count(&seats, row, seat, row_max, seat_max , false) {
                            0 => {
                                tmp[row][seat] = '#';
                                changed = true; 
                            },
                            _ => tmp[row][seat] = 'L'
                        }
                    },
                    '#' => {
                        match adj_count(&seats, row, seat, row_max, seat_max, false) {
                            0..=4 => tmp[row][seat] = '#',
                            5..=9 => {
                                tmp[row][seat] = 'L';
                                changed = true;
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
            } 
        }
        let mut line = String::new();
        if !skip {
                loop {
            std::io::stdin().read_line(&mut line);
            let input: Vec<&str> = line.strip_suffix("\n").unwrap().split_ascii_whitespace().collect();
            match input[0] {
                "g" => break,
                "c" => {
                        let coors: Vec<&str> = input[1].split(',').collect();
                        let row = coors[0].parse::<usize>().unwrap();
                        let seat = coors[1].parse::<usize>().unwrap();
                        println!("COUNT AT {},{} IS {}",row,seat, adj_count(&seats, row, seat, row_max, seat_max, true))
                },
                "p" =>{
                        for row in 0..row_max {
                            for seat in 0..seat_max {
                                print!("{}",seats[row][seat]);
                            } 
                            println!();
                        }
                },
                "f" => {
                    skip = true;
                    break;
                },
                _ => ()
            }

            line = String::new();
            }
        }
        seats = tmp;

         if !changed {
             break;
         }
       
        tmp = get_blank(seat_max, row_max);

    }
    let mut filled = 0;
    for row in 0..row_max {
        for seat in 0..seat_max {
            print!("{}",seats[row][seat]);
            if seats[row][seat] == '#' {
                filled += 1;
            }
        } 
        println!();
    }

    println!("FILLED: {}", filled);

    Ok(())
}

fn adj_count(seats: &Vec<Vec<char>>, row: usize, seat: usize, row_max: usize, seat_max: usize, diag: bool) -> usize {
    let mut count: usize = 0;
    
    for i in 1..row+1 {
            match seats[row-i][seat] {
                '#' => {
                    count += 1;
                    if diag {
                        print!("N,")
                    }
                    break;
                },
                'L' => break,
                '.' => (),
                _ => (),
            }
    }
    
    for i in row+1..row_max {
        match seats[i][seat] {
            '#' => {
                count += 1;
                if diag {
                    print!("S,")
                }
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }

    for i in 1..seat+1 {
        if diag { print!("{}",seats[row][i]);}
            match seats[row][seat-i] {
                '#' => {
                    count += 1;
                    if diag {
                        print!("W,")
                    }
                    break;
                },
                'L' => break,
                '.' => (),
                _ => (),
            }
    }
    
    for i in seat+1..seat_max {
        match seats[row][i]{
            '#' => {
                count += 1;
                if diag {
                    print!("E,")
                }
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }
  
   
    //SE Diagonal
    let range = cmp::min((seat_max-1 - seat), (row_max -1 - row));
    for i in 1..=range {
        match seats[row+i][seat+i] {
            '#' => {
                if diag {
                    print!("SE,")
                }
                count += 1;
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }
    //SW
    let range = cmp::min(seat, (row_max -1 - row));
    for i in 1..=range {
        match seats[row+i][seat-i]  {
            '#' => {
                count += 1;
                if diag {
                    print!("SW,")
                }
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }
    //NW
    let range = cmp::min(row,seat);
    for i in 1..=range {
        match seats[row-i][seat-i] {
            '#' => {
                count += 1;
                if diag {
                    print!("NW,")
                }
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }
    //NE
    let range = cmp::min(row,(seat_max-1 - seat));
    for i in 1..=range {
        match seats[row-i][seat+i]  {
            '#' => {
                if diag {
                    print!("NE,")
                }
                count += 1;
                break;
            },
            'L' => break,
            '.' => (),
            _ => (),
        }
    }
    if diag {
        println!();
    }
    return count;
}

fn is_seat(seat: &char) -> bool {
    match seat {
        '.' => return false,
        'L' => return false,
        '#' => return true,
        _ => return false
    }
}

fn get_blank(seat_max: usize, row_max: usize) -> Vec<Vec<char>> {
    let mut tmp: Vec<Vec<char>> = vec!();
    for _ in 0..row_max {
        let temp_row: Vec<char> = vec![' ';seat_max];
        tmp.push(temp_row);
    }

    return tmp;
}

fn read_data(filename: &str) -> (Vec<Vec<char>>, usize, usize) {
    let file = File::open(&filename).unwrap();
    let buffer = io::BufReader::new(file);
    let mut seats: Vec<Vec<char>> = vec!();
    let mut seat_max: usize = 0;
    let mut row_max: usize = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        row_max +=1;
        let row: Vec<char> = line.chars().collect();
        seat_max = row.len();
        seats.push(row);
    }

    return (seats,seat_max,row_max)
}