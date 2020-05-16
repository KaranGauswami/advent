#![allow(warnings)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let vectors = line?;

        let iterator = vectors.split(',');
        let mut vecs: Vec<i64> = Vec::new();
        for i in iterator {
            vecs.push(i.parse::<i64>().unwrap());
        }

        program_alarm(&mut vecs);
        // let mut i = 0;
    }

    Ok(())
}

fn generate_opt_codes(handle_code: &i64) -> Vec<i64> {
    let mut opt_code = *handle_code;
    let mut codes: Vec<i64> = Vec::new();
    codes.push(opt_code % 100);
    opt_code = opt_code / 100;
    for i in 0..3 {
        codes.push(opt_code % 10);
        opt_code = opt_code / 10;
    }
    // println!("{} generates {:?}", handle_code, codes);
    codes
}
fn program_alarm(array: &mut Vec<i64>) {
    let array_length = array.len();
    let mut i = 0;

    while i < array_length {
        let opt_code = generate_opt_codes(&array[i]);

        // println!("{:?}", opt_code);
        match opt_code[0] {
            1 => {
                let mut a: usize = 0;
                let mut b: usize = 0;
                let mut c: usize = 0;
                let mut d: usize = 0;
                let mut total: i64 = 0;
                if opt_code[1] == 0 {
                    a = array[i + 1] as usize;
                    c = array[a] as usize;
                } else {
                    c = array[i + 1] as usize;
                };
                if opt_code[2] == 0 {
                    b = array[i + 2] as usize;
                    d = array[b] as usize;
                } else {
                    d = array[i + 2] as usize;
                };
                total = c as i64 + d as i64;
                if opt_code[3] == 0 {
                    let index = array[i + 3] as usize;
                    array[index] = total as i64;
                } else {
                    array[i + 3] = total;
                };
                i += 4;
            }
            2 => {
                let mut a: usize = 0;
                let mut b: usize = 0;
                let mut c: usize = 0;
                let mut d: usize = 0;
                let mut total = 0;
                if opt_code[1] == 0 {
                    a = array[i + 1] as usize;
                    c = array[a] as usize;
                } else {
                    c = array[i + 1] as usize;
                };
                if opt_code[2] == 0 {
                    b = array[i + 2] as usize;
                    d = array[b] as usize;
                } else {
                    d = array[i + 2] as usize;
                };
                total = c as i64 * d as i64;
                if opt_code[3] == 0 {
                    let index = array[i + 3] as usize;
                    array[index] = total as i64;
                } else {
                    array[i + 3] = total;
                };
                i += 4;
            }
            3 => {
                let a = array[i + 1] as usize;
                array[a] = 5;
                i += 2;
            }
            4 => {
                if opt_code[1] == 0 {
                    println!("{}", array[array[i + 1] as usize]);
                } else {
                    println!("{}", array[i + 1]);
                }
                i += 2;
            }
            5 => {
                let mut first = 0;
                if opt_code[1] == 0 {
                    first = array[array[i + 1] as usize] as usize;
                } else {
                    first = array[i + 1] as usize;
                }

                if first != 0 {
                    if opt_code[2] == 0 {
                        i = array[array[i + 2] as usize] as usize;
                    } else {
                        i = array[i + 2] as usize;
                    }
                } else {
                    i += 3;
                }
            }
            6 => {
                let mut first = 0;
                if opt_code[1] == 0 {
                    first = array[array[i + 1] as usize];
                } else {
                    first = array[i + 1];
                }
                if first == 0 {
                    if opt_code[2] == 0 {
                        i = array[array[i + 2] as usize] as usize;
                    } else {
                        i = array[i + 2] as usize;
                    }
                } else {
                    i += 3;
                }
            }
            7 => {
                let mut first = 0;
                let mut second = 0;
                let mut third = 0;
                if opt_code[1] == 0 {
                    first = array[array[i + 1] as usize];
                } else {
                    first = array[i + 1];
                }
                if opt_code[2] == 0 {
                    second = array[array[i + 2] as usize];
                } else {
                    second = array[i + 2];
                }

                if first < second {
                    third = 1;
                }
                if opt_code[3] == 0 {
                    let a = array[i + 3] as usize;
                    array[a] = third;
                } else {
                    array[i + 3] = third;
                }
                i += 4;
            }
            8 => {
                let mut first = 0;
                let mut second = 0;
                let mut third = 0;
                if opt_code[1] == 0 {
                    first = array[array[i + 1] as usize];
                } else {
                    first = array[i + 1];
                }
                if opt_code[2] == 0 {
                    second = array[array[i + 2] as usize];
                } else {
                    second = array[i + 2];
                }

                if first == second {
                    third = 1;
                }
                if opt_code[3] == 0 {
                    let a = array[i + 3] as usize;
                    array[a] = third;
                } else {
                    array[i + 3] = third;
                }
                i += 4;
            }
            99 => {
                print!("HALT");
                break;
            }
            _ => {
                println!("{}", opt_code[0]);
                panic!("lol");
            }
        }
    }
}
