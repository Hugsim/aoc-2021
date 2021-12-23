#![allow(dead_code)]
#![allow(non_snake_case)]

mod days;
mod util;

fn main() {
    let solutions = days::day3::solve();

    if let Some(day1_solution) = solutions.0 {
        println!("Part 1: {}", day1_solution);
    }
    if let Some(day2_solution) = solutions.1 {
        println!("Part 2: {}", day2_solution);
    }
}
