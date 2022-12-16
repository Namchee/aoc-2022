use serde::{Deserialize};
use serde_json::from_str;

#[derive(Deserialize, PartialEq, Eq)]
#[serde(untagged)]

enum Token {
    Number(u32),
    Array(Vec<Token>),
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::Array(a), Self::Array(b)) => a.cmp(b),
            // why do we need to wrap `a` in Self::Number?
            // so the compiler can re-run the comparator function again
            // compiler doesn't know that `u32` is actually `Number`
            (Self::Number(a), Self::Array(b)) => vec![Self::Number(*a)].cmp(b),
            (Self::Array(a), Self::Number(b)) => a.cmp(&vec![Self::Number(*b)]),
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_one(input: Vec<String>) -> String {
    let chunks = parse_chunk(input);

    let mut sum = 0;

    for (idx, chunk) in chunks.iter().enumerate() {
        let json_a: Token = from_str(chunk[0].as_str()).unwrap();
        let json_b: Token = from_str(chunk[1].as_str()).unwrap();

        if json_a < json_b {
            sum += idx + 1;
        }
    }

    format!("{}", sum)
}

pub fn solve_two(_: Vec<String>) -> String {
    "".to_string()
}

fn parse_chunk(input: Vec<String>) -> Vec<Vec<String> > {
    let mut chunks: Vec<Vec<String> > = vec![];
    let mut temp = vec![];

    for line in input.iter() {
        if temp.len() == 2 {
            chunks.push(temp.clone());
            temp.clear();
            continue;
        }

        temp.push(line.clone());
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "13");
    }

    #[test]
    fn test_solve_two() {
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "");
    }
}