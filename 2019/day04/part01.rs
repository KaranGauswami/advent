#[allow(unused_imports)]
use std::collections::HashMap;

fn split_number(number: &u32) -> Vec<u32> {
    let mut number = number.clone();
    let mut number_vector: Vec<u32> = Vec::new();
    while number > 0 {
        let new_number = number % 10;
        number = number / 10;
        number_vector.push(new_number);
    }
    number_vector.reverse();
    number_vector
}
fn main() {
    let range_lower = 235741;
    let range_upper = 706948;
    let mut total_count_part_1 = 0;
    let mut total_count_part_2 = 0;

    for number in range_lower..=range_upper {
        let number_vector = split_number(&number);
        if is_number_valid(&number_vector) {
            total_count_part_1 += 1;
        }
        if is_number_valid(&number_vector) {
            total_count_part_2 += 1;
        }
    }
    println!("Total possible passwords are {}", total_count_part_1);
}
fn is_number_valid(number_vector: &Vec<u32>) -> bool {
    let mut adjacent = false;
    if number_vector[0] == number_vector[1]
        || number_vector[1] == number_vector[2]
        || number_vector[2] == number_vector[3]
        || number_vector[3] == number_vector[4]
        || number_vector[4] == number_vector[5]
    {
        adjacent = true
    }
    if adjacent != true {
        return false;
    }
    if number_vector[0] <= number_vector[1]
        && number_vector[1] <= number_vector[2]
        && number_vector[2] <= number_vector[3]
        && number_vector[3] <= number_vector[4]
        && number_vector[4] <= number_vector[5]
    {
        return true;
    }
    false
}
