use std::fs;

use colored::Colorize;
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

    let safe_report_count = reports
        .filter(|report| {
            let result = report
                .iter()
                .try_fold((None, None), |state, level| match state {
                    (None, _) => Ok((Some(level), None)),
                    (Some(previous_level), report_direction) => {
                        let direction = match level - previous_level {
                            1..=3 => Direction::Up,
                            -3..=-1 => Direction::Down,
                            difference => {
                                return Err(format!(
                                    "difference of {difference} at {previous_level} -> {level}"
                                ))
                            }
                        };
                        if let Some(report_direction) = report_direction {
                            if direction == report_direction {
                                Ok((Some(level), Some(direction)))
                            } else {
                                Err(format!("direction change at {previous_level} -> {level}"))
                            }
                        } else {
                            Ok((Some(level), Some(direction)))
                        }
                    }
                });
            let trace_message = if let Err(reason) = &result {
                format!("{}: {}", "UNSAFE".red(), reason)
            } else {
                "SAFE".green().to_string()
            };
            println!("{report:?}\t{trace_message}");
            result.is_ok()
        })
        .count();
    println!("{safe_report_count} safe reports found");
}
