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
            let result = check_report(report).map_err(|(reason, position)| {
                let remove_before = report.clone();
                remove_before.remove()(reason, position)
            });
            let trace_message = if let Err((reason, _)) = &result {
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

fn check_report(report: &Vec<i32>) -> Result<(), (String, usize)> {
    report
        .iter()
        .enumerate()
        .try_fold((None, None), |state, (position, level)| match state {
            (None, _) => Ok((Some(level), None)),
            (Some(previous_level), report_direction) => {
                let direction = match level - previous_level {
                    1..=3 => Direction::Up,
                    -3..=-1 => Direction::Down,
                    difference => {
                        return Err((
                            format!("difference of {difference} at {previous_level} -> {level}"),
                            position,
                        ))
                    }
                };
                if let Some(report_direction) = report_direction {
                    if direction == report_direction {
                        Ok((Some(level), Some(direction)))
                    } else {
                        Err((
                            format!("direction change at {previous_level} -> {level}"),
                            position,
                        ))
                    }
                } else {
                    Ok((Some(level), Some(direction)))
                }
            }
        })
        .map(|_| ())
}
