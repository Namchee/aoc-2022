use std::collections::HashSet;

// Where does this `96` coming from? ASCII values
// 97 (ascii value of 'a') - 96 = 1. This is the expected score
// 58 comes from 32 (diff between 'a' and 'A') + 26 (set of alphabet)

pub fn solve_one(input: Vec<String>) -> String {
    let mut sum: i32 = 0;

    for line in input.iter() {
        let first = line.chars().take(line.len() / 2).collect::<HashSet<char> >();
        let second = line.chars().skip(line.len() / 2).collect::<HashSet<char> >();
        
        let ch = first.intersection(&second).collect::<String>().chars().nth(0).unwrap();
    
        sum += ch as i32 - 96;
        if ch.to_ascii_lowercase() != ch {
            sum += 58;
        }
    }

    format!("{}", sum)
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut score = 0;
    for group in input.chunks(3) {
        let mut freq: [i32; 123] = [0; 123];

        for elf in group {
            let uq_char = elf.chars().collect::<HashSet<char> > ();
            for chr in uq_char {
                freq[chr as usize] += 1;
            }
        }

        let badge = freq.iter().position(|&x| x == 3).unwrap() as i32;
        score += badge - 96;
        if badge < 96 {
            score += 58;
        }
    }

    format!("{}", score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "157");
    }

    #[test]
    fn test_solve_two() {
        let str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "70");
    }
}