use std::fs;

use regex::Regex;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

fn main() {
    let pattern = Regex::new(r"\d+").unwrap();
    let input = fs::read_to_string("input").unwrap();
    let reports = input.lines().map(|levels| {
        pattern
            .captures_iter(levels)
            .map(|capture| capture[0].parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let safe_reports = reports.filter(|report| {
        report
            .iter()
            .try_fold((None, None), |state, level| match state {
                (None, _) => Ok((Some(level), None)),
                (Some(previous_level), report_direction) => {
                    let direction = match level - previous_level {
                        1..=3 => Direction::Up,
                        -3..=-1 => Direction::Down,
                        difference => return Err(String::from("difference of {difference}")),
                    };
                    if let Some(report_direction) = report_direction {
                        if direction == report_direction {
                            Ok((Some(level), Some(direction)))
                        } else {
                            Err(String::from(
                                "direction change at {previous_level} -> {level}",
                            ))
                        }
                    } else {
                        Ok((Some(level), Some(direction)))
                    }
                }
            })
            .is_ok()
    });
}
