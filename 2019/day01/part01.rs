use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel_required = 0;

    for line in reader.lines() {
        let mass = line?.parse::<u32>().unwrap();
        let fuel_required = (mass / 3) - 2;
        total_fuel_required += fuel_required;
    }
    println!("Total Fuel Required is {}", total_fuel_required);

    Ok(())
}
