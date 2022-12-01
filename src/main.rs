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
    let input_file = utils::read_input(day);

    if input_file.is_err() {
        println!("❌ {}", input_file.unwrap_err());
        process::exit(0);
    }

    let input = input_file.unwrap();

    match day {
        1 => solve!(solutions::day01, input.clone()),
        _ => println!("Not solved yet..."),
    }
}
