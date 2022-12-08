use std::cmp::max;

pub fn solve_one(input: Vec<String>) -> String {
    let (w, h) = (input[0].len(), input.len());

    let mut source: Vec<Vec<u32> > = vec![vec![0; w]; h];
    for (i, val) in input.iter().enumerate() {
        for (j, c) in val.chars().enumerate() {
            source[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut left: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut right: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut top: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut bottom: Vec<Vec<u32> > = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 1..w - 1 {
            left[i][j] = max(left[i][j - 1], source[i][j - 1]);
            right[i][w - 1 - j] = max(right[i][w - j], source[i][w - j]); 
        }
    }

    for i in 0..w {
        for j in 1..h - 1 {
            top[j][i] = max(top[j - 1][i], source[j - 1][i]);
            bottom[h - 1 - j][i] = max(bottom[h - j][i], source[h - j][i]); 
        }
    }

    let mut sum: u32 = 0;

    for i in 0..h {
        for j in 0..w {
            if  i == 0 || j == 0 || i == h - 1 || j == w - 1 ||
                source[i][j] > left[i][j] || source[i][j] > right[i][j] || source[i][j] > top[i][j] || source[i][j] > bottom[i][j] {
                sum += 1;
            }
        }
    }

    format!("{}", sum)
}

pub fn solve_two(input: Vec<String>) -> String {
    let (w, h) = (input[0].len(), input.len());

    let mut source: Vec<Vec<u32> > = vec![vec![0; w]; h];
    for (i, val) in input.iter().enumerate() {
        for (j, c) in val.chars().enumerate() {
            source[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut left: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut right: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut top: Vec<Vec<u32> > = vec![vec![0; w]; h];
    let mut bottom: Vec<Vec<u32> > = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 1..w - 1 {
            left[i][j] = 1;
            if source[i][j - 1] < source[i][j] {
                left[i][j] += left[i][j - 1];
            }

            right[i][w - 1 - j] = 1;
            if source[i][w - j] < source[i][w - 1 - j] {
                right[i][w - 1 - j] += right[i][w - j];
            }
        }
    }

    for i in 0..w {
        for j in 1..h - 1 {
            top[j][i] = 1;
            if source[j - 1][i] < source[j][i] {
                top[j][i] += top[j - 1][i];
            }

            bottom[h - 1 - j][i] = 1;
            if source[h - j][i] < source[h - 1 - j][i] {
                bottom[h - 1 - j][i] += bottom[h - j][i];
            }
        }
    }

    let mut best: u32 = 0;

    for i in 0..h {
        for j in 0..w {
            best = max(best, left[i][j] * right[i][j] * top[i][j] * bottom[i][j]);
        }
    }

    format!("{}", best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "30373
25512
65332
33549
35390";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "21");
    }

    #[test]
    fn test_solve_two() {
        let str = "30373
25512
65332
33549
35390";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "8");
    }
}