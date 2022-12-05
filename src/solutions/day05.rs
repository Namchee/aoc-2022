use regex::Regex;

pub fn solve_one(input: Vec<String>) -> String {
    let mut tops: Vec<char> = vec![];

    let mut stacks = get_stacks(input.clone());
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    
    for line in input.iter() {
        let res = pattern.captures(line);

        if res.is_some() {
            let captured = res.unwrap();

            let much = &captured[1].parse::<u32>().unwrap();
            let from = &captured[2].parse::<usize>().unwrap() - 1;
            let to = &captured[3].parse::<usize>().unwrap() - 1;

            for _ in 0..*much {
                let chr = stacks[from].pop().unwrap();
                stacks[to].push(chr);
            }
        }
    }

    for stack in stacks.iter() {
        tops.push(*stack.last().unwrap());
    }

    format!("{}", tops.into_iter().collect::<String>())
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut tops: Vec<char> = vec![];

    let mut stacks = get_stacks(input.clone());
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    
    for line in input.iter() {
        let res = pattern.captures(line);

        if res.is_some() {
            let captured = res.unwrap();

            let much = &captured[1].parse::<usize>().unwrap();
            let from = &captured[2].parse::<usize>().unwrap() - 1;
            let to = &captured[3].parse::<usize>().unwrap() - 1;

            let ln = stacks[from].len();

            let mut chrs = stacks[from].split_off(ln - much);
            stacks[to].append(&mut chrs);
        }
    }

    for stack in stacks.iter() {
        tops.push(*stack.last().unwrap());
    }

    format!("{}", tops.into_iter().collect::<String>())
}

fn get_stacks(input: Vec<String>) -> Vec<Vec<char> > {
    let length = (input[0].len() as f32 / 4.0).ceil() as usize;
    let mut result: Vec<Vec<char> > = vec![vec![]; length];

    for line in input.iter() {
        let chara = line.chars().collect::<Vec<char> >();

        if chara[1] == '1' {
            break;
        }

        for idx in (1..chara.len()).step_by(4) {
            if !chara[idx].is_whitespace() {
                let target = (idx as f32 / 4.0).floor() as usize;
                result[target].push(chara[idx]);
            }
        }
    }
    
    for idx in 0..result.len() {
       result[idx].reverse();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "CMZ");
    }

    #[test]
    fn test_solve_two() {
        let str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "MCD");
    }
}