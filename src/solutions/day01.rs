use std::cmp;

pub fn solve_one(input: Vec<String>) -> String {
    let mut cur: u32 = 0;
    let mut best: u32 = 0;

    for value in input.iter() {
        if value.len() == 0 {
            best = cmp::max(cur, best);
            cur = 0;
            continue;
        }
        let val = value.parse::<u32>().unwrap();
        cur += val;
    }

    best = cmp::max(cur, best);

    return format!("{}", best);
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut cur: u32 = 0;
    
    let mut sum: Vec<u32> = vec![];

    for value in input.iter() {
        if value.len() == 0 {
            sum.push(cur);
            cur = 0;
            continue;
        }
        let val = value.parse::<u32>().unwrap();
        cur += val;
    }

    sum.push(cur);
    sum.sort();

    let result = sum[sum.len() - 1] + sum[sum.len() - 2] + sum[sum.len() - 3];

    return format!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "24000".to_string())
    }

    #[test]
    fn test_solve_two() {
        let str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();
        assert_eq!(solve_two(input), "45000".to_string())
    }
}