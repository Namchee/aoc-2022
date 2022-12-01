pub fn solve_one(input: Vec<String>) -> String {
    return  "hello world".to_string();
}

pub fn solve_two(input: Vec<String>) -> String {
    return "goodbye ".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        assert_eq!(solve_one(vec![]), "".to_string())
    }

    #[test]
    fn test_solve_two() {
        assert_eq!(solve_two(vec![]), "".to_string())
    }
}