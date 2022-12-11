use std::collections::HashSet;

pub fn solve_one(input: Vec<String>) -> String {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::from([
        (0, 0),
    ]);

    for line in input.iter() {
        let tokens = line.split(" ").collect::<Vec<&str> >();
        let dir = get_dir(tokens[0]);
        let mut count = tokens[1].parse::<u32>().unwrap();

        while count > 0 {
            head.0 += dir.0;
            head.1 += dir.1;

            tail = get_tail_position(head, tail);
            visited.insert(tail);

            count -= 1;
        }
    }

    format!("{}", visited.len())
}

pub fn solve_two(_: Vec<String>) -> String {
    "".to_string()
}

fn get_dir(dir: &str) -> (i32, i32) {
    match dir {
        "U" => (0, -1),
        "D" => (0, 1),
        "L" => (-1, 0),
        _ => (1, 0),
    }
}

fn get_tail_position(
    head: (i32, i32),
    tail: (i32, i32),
) -> (i32, i32) {
    let mut new_pos = (tail.0, tail.1);
    let diff = (head.0 - tail.0, head.1 - tail.1);

    if diff.0 > 1 {
        new_pos.0 += 1;
        new_pos.1 += diff.1;
    }
    if diff.0 < -1 {
        new_pos.0 -= 1;
        new_pos.1 += diff.1;
    }
    if diff.1 > 1 {
        new_pos.1 += 1;
        new_pos.0 += diff.0;
    }
    if diff.1 < -1 {
        new_pos.1 -= 1;
        new_pos.0 += diff.0;
    }

    return new_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    
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