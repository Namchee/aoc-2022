use std::env;
use std::process;

mod utils;
mod solutions;

macro_rules! solve{
    ($day: path, $input: expr) => {
        {
            use $day::{solve_one, solve_two};

            println!("*** PART ONE ***");
            println!("");
            println!("{}", solve_one($input));
            println!("");
            println!("*** PART TWO ***");
            println!("");
            println!("{}", solve_two($input));
            println!("");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().unwrap();
    let input = utils::read_input(day);

    if input.is_err() {
        println!("❌ {}", input.unwrap_err());
        process::exit(0);
    }

    match day {
        1 => solve!(solutions::day01, input.as_ref().unwrap().clone()),
        2 => solve!(solutions::day02, input.as_ref().unwrap().clone()),
        3 => solve!(solutions::day03, input.as_ref().unwrap().clone()),
        4 => solve!(solutions::day04, input.as_ref().unwrap().clone()),
        5 => solve!(solutions::day05, input.as_ref().unwrap().clone()),
        6 => solve!(solutions::day06, input.as_ref().unwrap().clone()),
        7 => solve!(solutions::day07, input.as_ref().unwrap().clone()),
        8 => solve!(solutions::day08, input.as_ref().unwrap().clone()),
        9 => solve!(solutions::day09, input.as_ref().unwrap().clone()),
        10 => solve!(solutions::day10, input.as_ref().unwrap().clone()),
        _ => println!("Not solved yet..."),
    }
}
