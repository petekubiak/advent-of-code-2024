use std::fs;

use colored::Colorize;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let input = fs::read_to_string("input").unwrap();
    let sum = pattern
        .captures_iter(&input)
        .fold((0, true), |(sum, enabled), capture| {
            print!("Found {:10} ", &capture[0]);
            match &capture[0] {
                "do()" => {
                    println!("{}", "Enabling...".green());
                    (sum, true)
                }
                "don't()" => {
                    println!("{}", "Disabling...".red());
                    (sum, false)
                }
                _ => {
                    if enabled {
                        let product =
                            capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
                        let new_sum = sum + product;
                        println!(
                            "{:4} * {:4} = {product}; sum: {new_sum}",
                            &capture[1], &capture[2]
                        );
                        (new_sum, enabled)
                    } else {
                        println!("{}", "IGNORED".red());
                        (sum, enabled)
                    }
                }
            }
        });
    println!("Final sum: {}", sum.0);
}
