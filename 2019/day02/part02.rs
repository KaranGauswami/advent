use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let vectors = line?;

        let iterator = vectors.split(',');
        let mut vecs: Vec<u32> = Vec::new();
        for i in iterator {
            vecs.push(i.parse::<u32>().unwrap());
        }
        // let mut vecs: Vec<u32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        for i in 0..=100 {
            for j in 0..=100 {
                let mut lol = vecs.clone();
                program_alarm(&mut lol, i, j);
                if (lol[0] == 19690720) {
                    println!("{} {} {}", lol[0], i, j);
                    println!("Answer is {} ", 100 * i + j)
                }
            }
        }
    }

    Ok(())
}
fn program_alarm(array: &mut Vec<u32>, i: u32, j: u32) {
    let array_length = array.len();
    let mut counter = 0;

    array[1] = i;
    array[2] = j;
    while counter < array_length {
        match array[counter] {
            1 => {
                let a: usize = array[counter + 1] as usize;
                let b: usize = array[counter + 2] as usize;
                let c: usize = array[a] as usize;
                let d: usize = array[b] as usize;
                let total = c as u32 + d as u32;
                let index = array[counter + 3] as usize;
                array[index] = total as u32;
            }
            2 => {
                let a: usize = array[counter + 1] as usize;
                let b: usize = array[counter + 2] as usize;
                let c: usize = array[a] as usize;
                let d: usize = array[b] as usize;
                let total = c as u32 * d as u32;
                let index = array[counter + 3] as usize;
                array[index] = total as u32;
            }
            99 => {
                break;
            }
            _ => {}
        }
        counter += 4;
    }
}
