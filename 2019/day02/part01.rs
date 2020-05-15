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
        program_alarm(&mut vecs);
        println!("Value at position zero is {}", vecs[0]);
    }

    Ok(())
}
fn program_alarm(array: &mut Vec<u32>) {
    let array_length = array.len();
    let mut counter = 0;
    array[1] = 53;
    array[2] = 98;
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
