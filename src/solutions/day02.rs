use std::collections::HashMap;

pub fn solve_one(input: Vec<String>) -> String {
    let score_map: HashMap<String, u32> = HashMap::from([
        ("A X".to_string(), 4),
        ("A Y".to_string(), 8),
        ("A Z".to_string(), 3),

        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),

        ("C X".to_string(), 7),
        ("C Y".to_string(), 2),
        ("C Z".to_string(), 6),
    ]);

    let mut score: u32 = 0;

    for round in input.iter() {
        score += score_map.get(round).unwrap();
    }

    format!("{}", score)
}

pub fn solve_two(input: Vec<String>) -> String {
    let score_map: HashMap<String, u32> = HashMap::from([
        ("A X".to_string(), 3),
        ("A Y".to_string(), 4),
        ("A Z".to_string(), 8),

        ("B X".to_string(), 1),
        ("B Y".to_string(), 5),
        ("B Z".to_string(), 9),

        ("C X".to_string(), 2),
        ("C Y".to_string(), 6),
        ("C Z".to_string(), 7),
    ]);

    let mut score: u32 = 0;

    for round in input.iter() {
        score += score_map.get(round).unwrap();
    }

    format!("{}", score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "A Y
B X
C Z";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "15");
    }

    #[test]
    fn test_solve_two() {
        let str = "A Y
B X
C Z";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "12");
    }
}