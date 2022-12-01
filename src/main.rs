use std::{env, fs, process};

pub mod day1;

fn main() {
    let Some(command) = env::args().nth(1) else {
        println!("Usage: aoc day1");
        process::exit(1);
    };
    let day1 = day1::Day1::new();
    let input = fs::read_to_string(command).unwrap();
    let max = day1.which_elve_max_calory(&input);
    println!("day1a: {}", max);
    let max = day1.top_3_elves_v1(&input);
    println!("day1b: {}", max);
}
