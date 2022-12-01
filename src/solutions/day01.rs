use std::cmp;

pub fn solve_one(input: Vec<String>) -> String {
    let chunks = split_to_chunks(input);
    let mut best: u32 = 0;

    for chunk in chunks.iter() {
        best = cmp::max(chunk.iter().sum(), best);
    }

    return format!("{}", best);
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut chunks: Vec<u32> = split_to_chunks(input)
        .iter()
        .map(|x| x.iter().sum())
        .collect();
    chunks.sort();
    chunks.reverse();

    return format!("{}", chunks[0] + chunks[1] + chunks[2]);
}

fn split_to_chunks(input: Vec<String>) -> Vec<Vec<u32> > {
    let mut result: Vec<Vec<u32> > = vec![];
    let mut temp: Vec<u32>= vec![];

    for val in input {
        if val.is_empty() {
            result.push(temp.clone());
            temp.clear();
            continue;
        }

        temp.push(val.parse::<u32>().unwrap());
    }

    result.push(temp);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_to_chunks() {
        let str = "1000
2000
3000

4000";

        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(split_to_chunks(input), vec![vec![1000, 2000, 3000], vec![4000]]);
    }

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