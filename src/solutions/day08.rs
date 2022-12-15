use std::cmp::max;

type Position = (i32, i32);

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

    let mut best: u32 = 0;

    for (i, row) in source.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let score = get_scenic_score(source.clone(), (i as i32, j as i32));

            best = max(best, score);
        }
    }

    format!("{}", best)
}

fn get_scenic_score(grid: Vec<Vec<u32> >, pos: Position) -> u32 {
    let directions: Vec<Position> = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
    ];

    let mut count = vec![];

    let dim: Position = (grid.len() as i32, grid[0].len() as i32);

    for (x, y) in directions.iter() {
        let mut curr = (
            pos.0 + x,
            pos.1 + y,
        );

        let mut count_dir = 0;

        while in_bound(dim, curr) {
            count_dir += 1;

            if grid[pos.0 as usize][pos.1 as usize] > grid[curr.0 as usize][curr.1 as usize] {
                curr.0 += x;
                curr.1 += y;
            } else {
                break;
            }
        }

        count.push(count_dir);
    }

    count.iter().product()
}

fn in_bound(dim: Position, pos: Position) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < dim.0 && pos.1 < dim.1
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