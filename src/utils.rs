use std::fs;
use std::env;

pub fn read_input(day: u8) -> Result<Vec<String>, String> {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("input")
        .join(format!("day{:02}.txt", day));

    if !filepath.exists() {
        return Err("Input file does not exist".to_string());
    }

    Ok(fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect())
}