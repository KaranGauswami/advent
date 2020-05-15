use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut its = reader.lines();
    let line1 = its.next().unwrap().unwrap();
    let line2 = its.next().unwrap().unwrap();
    let line1iter = line1.split(',');
    let line2iter = line2.split(',');
    let mut first_points: Vec<&str> = Vec::new();
    let mut second_points: Vec<&str> = Vec::new();
    for i in line1iter {
        first_points.push(i);
    }
    for i in line2iter {
        second_points.push(i);
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut points_a = vec![(x, y)];
    let mut points_b = vec![(x, y)];

    let mut points_a_hash = HashSet::new();

    for i in first_points {
        let direction = &i[0..1];
        let step_count = &i[1..].to_owned().parse::<i32>().unwrap();

        for _ in 0..*step_count {
            match direction {
                "L" => x -= 1,
                "R" => x += 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => panic!("Invalid Input"),
            }
            points_a.push((x, y));
            points_a_hash.insert((x, y));
        }
    }
    x = 0;
    y = 0;

    let mut minimum_distance = std::i32::MAX;
    let mut intersection_points: Vec<(i32, i32)> = Vec::new();

    for i in second_points {
        let direction = &i[0..1];
        let step_count = &i[1..].to_owned().parse::<i32>().unwrap();
        for _ in 0..*step_count {
            match direction {
                "L" => x -= 1,
                "R" => x += 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => panic!("Invalid Input"),
            }
            points_b.push((x, y));
            if points_a_hash.contains(&(x, y)) {
                intersection_points.push((x, y));
                let a = x.abs();
                let b = y.abs();

                if a + b < minimum_distance {
                    minimum_distance = a + b;
                }
            };
        }
    }
    let mut minimum_steps = std::i32::MAX;
    for i in intersection_points {
        let index_a = points_a
            .iter()
            .position(|&(x, y)| x == i.0 && y == i.1)
            .unwrap();
        let index_b = points_b
            .iter()
            .position(|&(x, y)| x == i.0 && y == i.1)
            .unwrap();
        if minimum_steps > index_a as i32 + index_b as i32 {
            minimum_steps = index_a as i32 + index_b as i32;
        };
    }

    println!("Minimum distance is {}", minimum_distance);
    println!("Minimum steps is {} ", minimum_steps);
    Ok(())
}
