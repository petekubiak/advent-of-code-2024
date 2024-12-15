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
            println!();
            let result = check_report(&mut report.iter())
                .inspect_err(|(message, _)| println!("{message}"))
                .map_or_else(
                    |(message, position)| {
                        let position_before = position - 1;
                        check_report(
                            &mut report
                                .iter()
                                .enumerate()
                                .filter(|(index, _)| *index != position_before)
                                .map(|(_, level)| level),
                        )
                        .inspect_err(|(message, _)| println!("{message}"))
                        .map_or_else(
                            |_| {
                                check_report(
                                    &mut report
                                        .iter()
                                        .enumerate()
                                        .filter(|(index, _)| *index != position)
                                        .map(|(_, level)| level),
                                )
                                .map_or_else(
                                    |_| {
                                        check_report(&mut report.iter().skip(1))
                                            .map(|_| made_safe_report(report, 0))
                                    },
                                    |_| Ok(made_safe_report(report, position)),
                                )
                            },
                            |_| Ok(made_safe_report(report, position_before)),
                        )
                        .inspect_err(|(message, _)| println!("{message}"))
                        .or(Err((message, position)))
                    },
                    |_| Ok("SAFE".green().to_string()),
                );

            let trace_message = match &result {
                Ok(message) => message,
                Err((message, _)) => message,
            };
            println!("\n{report:?}\t{trace_message}");
            result.is_ok()
        })
        .count();
    println!("{safe_report_count} safe reports found");
}

fn check_report(report: &mut dyn Iterator<Item = &i32>) -> Result<(), (String, usize)> {
    report
        .inspect(|level| print!("{} ", level))
        .enumerate()
        .try_fold((None, None), |state, (position, level)| match state {
            (None, _) => Ok((Some(level), None)),
            (Some(previous_level), report_direction) => {
                let direction = match level - previous_level {
                    1..=3 => Direction::Up,
                    -3..=-1 => Direction::Down,
                    difference => {
                        return Err((
                            format!(
                                "{}: difference of {difference} at {previous_level} -> {level}",
                                "UNSAFE".red()
                            ),
                            position,
                        ))
                    }
                };
                if let Some(report_direction) = report_direction {
                    if direction == report_direction {
                        Ok((Some(level), Some(direction)))
                    } else {
                        Err((
                            format!(
                                "{}: direction change at {previous_level} -> {level}",
                                "UNSAFE".red()
                            ),
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

fn made_safe_report(report: &[i32], position: usize) -> String {
    format!(
        "{}: removed {} at position {}",
        "MADE SAFE".blue(),
        report[position],
        position
    )
}
