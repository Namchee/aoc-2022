use std::env;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().unwrap();
    let input = utils::read_input(day);

    // println!("{}", input.join("\n"));
}
