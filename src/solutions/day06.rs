use std::collections::HashSet;

pub fn solve_one(input: Vec<String>) -> String {
    let mut pos: usize = 0;
    let str = &input[0];

    for idx in 3..str.len() {
        let seq = str.chars().skip(idx - 3).take(4).collect::<HashSet<char> >();

        if seq.len() == 4 {
            pos = idx + 1;
            break;
        }
    }

    format!("{}", pos)
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut pos: usize = 0;
    let str = &input[0];

    for idx in 13..str.len() {
        let seq = str.chars().skip(idx - 13).take(14).collect::<HashSet<char> >();

        if seq.len() == 14 {
            pos = idx + 1;
            break;
        }
    }

    format!("{}", pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one_tc_one() {
        let str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "7");
    }

    #[test]
    fn test_solve_one_tc_two() {
        let str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "5");
    }

    #[test]
    fn test_solve_one_tc_three() {
        let str = "nppdvjthqldpwncqszvftbrmjlhg";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "6");
    }

    #[test]
    fn test_solve_two_tc_one() {
        let str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "19");
    }

    #[test]
    fn test_solve_two_tc_two() {
        let str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "23");
    }

    #[test]
    fn test_solve_two_tc_three() {
        let str = "nppdvjthqldpwncqszvftbrmjlhg";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "23");
    }
}