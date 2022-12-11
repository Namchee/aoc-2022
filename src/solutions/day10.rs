pub fn solve_one(input: Vec<String>) -> String {
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;

    let mut result: i32 = 0;
    let mut target: i32 = 20;

    for line in input.iter() {
        if cycle > 220 {
            break;
        }

        let tokens = line.split(" ").collect::<Vec<_> >();
        match tokens[0] {
            "addx" => {
                let val = tokens[1].parse::<i32>().unwrap();
                cycle += 1;

                if cycle == target {
                    result += register * target;

                    target += 40;
                }

                cycle += 1;
                if cycle == target {
                    result += register * target;

                    target += 40;
                }

                register += val;
            },
            _ => {
                cycle += 1;
                if cycle == target {
                    result += register * target;

                    target += 40;
                }
            },
        }
    }

    format!("{}", result)
}

pub fn solve_two(_: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "13140");
    }

    #[test]
    fn test_solve_two() {
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "");
    }
}