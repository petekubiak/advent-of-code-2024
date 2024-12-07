use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let pattern = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                captures[1].parse::<u32>().unwrap(),
                captures[2].parse::<u32>().unwrap(),
            )
        })
        .collect();

    first.sort();
    second.sort();

    let distance = first
        .iter()
        .zip(&second)
        .fold(0, |sum, (first, second)| sum + first.abs_diff(*second));

    println!("Distance between the lists: {:?}", distance);

    let second_as_hashmap = second.iter().fold(HashMap::new(), |mut map, key| {
        if let Some(count) = map.get_mut(key) {
            *count += 1;
        } else {
            map.insert(*key, 1);
        }
        map
    });
    let similarity = first.into_iter().fold(0, |sum, element| 
        sum + element * second_as_hashmap.get(&element).copied().unwrap_or_default());

    println!("Similarity between the lists: {:?}", similarity);
}
