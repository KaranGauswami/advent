use std::fs::File;
use std::io::{self, prelude::*, BufReader};
#[derive(Debug, PartialEq)]
struct Planet {
    value: String,
    child: String,
}
fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut planet_array: Vec<String> = Vec::new();
    for line in reader.lines() {
        let vectors = line?;
        println!("{}", vectors);
        planet_array.push(vectors)
    }

    let mut planet_nodes: Vec<Planet> = Vec::new();
    for i in planet_array {
        let split = i.split(")");
        let vec: Vec<&str> = split.collect();
        planet_nodes.push(Planet {
            value: vec[0].to_owned(),
            child: vec[1].to_owned(),
        });
    }
    let mut total_childs = 0;
    for i in &planet_nodes {
        let childs = count_child(i, &planet_nodes);
        total_childs += childs;
    }
    println!("Total childs are {}", total_childs);
    Ok(())
}
fn count_child(i: &Planet, total_planet: &Vec<Planet>) -> u32 {
    let mut children_count = 0;
    for j in total_planet {
        if i.child == j.value {
            children_count += count_child(j, total_planet);
        }
    }
    children_count + 1
}
