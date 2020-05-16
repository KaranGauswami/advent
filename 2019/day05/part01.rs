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
        let mut counter = 0;
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
    let mut counter = 0;

    while counter < array_length {
        let opt_code = generate_opt_codes(&array[counter]);

        // println!("{:?}", opt_code);
        match opt_code[0] {
            1 => {
                let mut a: usize = 0;
                let mut b: usize = 0;
                let mut c: usize = 0;
                let mut d: usize = 0;
                let mut total: i64 = 0;
                if opt_code[1] == 0 {
                    a = array[counter + 1] as usize;
                    c = array[a] as usize;
                } else {
                    c = array[counter + 1] as usize;
                };
                if opt_code[2] == 0 {
                    b = array[counter + 2] as usize;
                    d = array[b] as usize;
                } else {
                    d = array[counter + 2] as usize;
                };
                total = c as i64 + d as i64;
                if opt_code[3] == 0 {
                    let index = array[counter + 3] as usize;
                    array[index] = total as i64;
                } else {
                    array[counter + 3] = total;
                };
                counter += 4;
            }
            2 => {
                let mut a: usize = 0;
                let mut b: usize = 0;
                let mut c: usize = 0;
                let mut d: usize = 0;
                let mut total = 0;
                if opt_code[1] == 0 {
                    a = array[counter + 1] as usize;
                    c = array[a] as usize;
                } else {
                    c = array[counter + 1] as usize;
                };
                if opt_code[2] == 0 {
                    b = array[counter + 2] as usize;
                    d = array[b] as usize;
                } else {
                    d = array[counter + 2] as usize;
                };
                total = c as i64 * d as i64;
                if opt_code[3] == 0 {
                    let index = array[counter + 3] as usize;
                    array[index] = total as i64;
                } else {
                    array[counter + 3] = total;
                };
                counter += 4;
            }
            3 => {
                let a = array[counter + 1] as usize;
                array[a] = 1;
                counter += 2;
            }
            4 => {
                if opt_code[1] == 0 {
                    println!("{}", array[array[counter + 1] as usize]);
                } else {
                    println!("{}", array[counter + 1]);
                }
                counter += 2;
            }
            99 => {
                break;
            }
            _ => {
                panic!("panic");
            }
        }
    }
}
