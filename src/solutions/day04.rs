use std::cmp;

pub fn solve_one(input: Vec<String>) -> String {
    let mut overlaps = 0;

    for line in input.iter() {
        let tokens: Vec<_> = line.split(",").collect();

        let first: Vec<_> = tokens[0].split("-").collect();
        let second: Vec<_> = tokens[1].split("-").collect();

        let pair_a: Vec<_> = first.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let pair_b: Vec<_> = second.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let pair_c = vec![
            cmp::min(pair_a[0], pair_b[0]),
            cmp::max(pair_a[1], pair_b[1]),
        ];

        if (pair_c[0] == pair_a[0] && pair_c[1] == pair_a[1]) || (pair_c[0] == pair_b[0] && pair_c[1] == pair_b[1]) {
            overlaps += 1;
        }
    }

    format!("{}", overlaps)
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut overlaps = 0;

    for line in input.iter() {
        let tokens: Vec<_> = line.split(",").collect();

        let first: Vec<_> = tokens[0].split("-").collect();
        let second: Vec<_> = tokens[1].split("-").collect();

        let pair_a: Vec<_> = first.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let pair_b: Vec<_> = second.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        if (pair_a[0] >= pair_b[0] && pair_a[0] <= pair_b[1]) ||
            (pair_a[1] >= pair_b[0] && pair_a[1] <= pair_b[1]) ||
            (pair_b[0] >= pair_a[0] && pair_b[0] <= pair_a[1]) ||
            (pair_b[1] >= pair_a[0] && pair_b[1] <= pair_a[1]) {
            overlaps += 1;
        }
    }

    format!("{}", overlaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "2");
    }

    #[test]
    fn test_solve_two() {
        let str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "4");
    }
}