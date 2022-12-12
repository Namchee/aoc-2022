pub fn solve_one(_: Vec<String>) -> String {
    "".to_string()
}

pub fn solve_two(_: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "");
    }

    #[test]
    fn test_solve_two() {
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "");
    }
}