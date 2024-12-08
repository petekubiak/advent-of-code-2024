use std::fs;

use regex::Regex;

fn main() {
    let pattern = Regex::new(r"\d+").unwrap();
    // let first_line = fs::read_to_string("input")
    //     .unwrap()
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .to_owned();
    // println!("{}", first_line);
    // let caps = pattern.captures_iter(&first_line);
    // for num in caps {
    //     println!("{:?}", num);
    // }
    let input = fs::read_to_string("input").unwrap();
    let reports = input
        .lines()
        .map(|levels| {
                pattern
                    .captures_iter(levels)
                    .map(|capture| capture[0].parse::<i32>().unwrap())
        });
    // println!("{:#?}", reports);

    let safe_reports = reports
        .filter(|levels| )
}
