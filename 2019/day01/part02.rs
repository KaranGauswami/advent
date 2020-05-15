use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel_required = 0;

    for line in reader.lines() {
        let mass = line?.parse::<i32>().unwrap();
        total_fuel_required += get_fuel_requirements(mass);
    }
    println!("Total Fuel Required is {}", total_fuel_required);
    Ok(())
}
fn get_fuel_requirements(mass: i32) -> i32 {
    let mut total_fuel_required = 0;
    let mut fuel_required = mass;
    while fuel_required > 0 {
        fuel_required = (fuel_required / 3) - 2;
        if fuel_required > 0 {
            total_fuel_required += fuel_required;
        }
    }
    total_fuel_required
}
