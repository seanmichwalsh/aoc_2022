use crate::solutions::*;

mod solutions;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Error: Please include the day to solve for\n");
    }

    let day = &args[1].parse::<u32>().unwrap();
    let result = match day {
        1 => day1::solve(),
        2..=25 => Err("Error: Day not yet implemented".into()),
        _ => Err("Error: Advent of Code only runs from Dec 1-25".into()),
    };

    match result {
        Ok(answer) => {
            println!("Answers for day {}:", day);
            println!("Part 1: {}", answer.0);
            println!("Part 2: {}", answer.1);
        },
        Err(err) => panic!("Error: {}\n", err),
    };
}
