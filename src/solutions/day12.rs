use std::collections::VecDeque;
use std::cmp::min;

type Position = (usize, usize);

struct Memo {
    pos: Position,
    walk: usize,
}

pub fn solve_one(input: Vec<String>) -> String {
    let (grid, start, end) = to_grid(input);
    let best = bfs(grid, start, end);

    format!("{}", best)
}

// needs some optimization
// still quite fast, but can be optimized
pub fn solve_two(input: Vec<String>) -> String {
    let (grid, start, end) = to_grid_two(input);

    let mut best = grid.len() * grid[0].len();
    for pos in start.iter() {
        let curr = bfs(grid.clone(), *pos, end);

        best = min(best, curr);
    }

    format!("{}", best)
}

fn bfs(grid: Vec<Vec<char> >, start: Position, end: Position) -> usize {
    let direction: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut queue: VecDeque<Memo> = VecDeque::from([
        Memo{pos: start, walk: 0},
    ]);

    let mut best = grid.len() * grid[0].len();

    while queue.len() > 0 {
        let position = queue.pop_front().unwrap();
        if position.pos == end {
            best = min(position.walk, best);
            continue;
        }

        if visited[position.pos.0][position.pos.1] {
            continue;
        }
        visited[position.pos.0][position.pos.1] = true;
        
        for dir in direction.iter() {
            let x = position.pos.0 as i32 + dir.0;
            let y = position.pos.1 as i32 + dir.1;

            let bound = x >= 0 && (x as usize) < grid.len() && y >= 0 && (y as usize) < grid[0].len();

            if bound && 
                !visited[x as usize][y as usize] &&
                can_move(grid[position.pos.0][position.pos.1], grid[x as usize][y as usize]) {
                queue.push_back(Memo{
                    pos: (x as usize, y as usize),
                    walk: position.walk + 1,
                });
            }
        }
    }

    best
}

fn can_move(a: char, b: char) -> bool {
    let old_val = a as u8;
    let new_val = b as u8;

    return new_val <= old_val || (new_val - old_val <= 1);
}

fn to_grid(input: Vec<String>) -> (Vec<Vec<char> >, Position, Position) {
    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);

    let mut grid: Vec<Vec<char> > = vec![];

    for (i, line) in input.iter().enumerate() {
        let mut current = vec![];

        for (j, c) in line.chars().enumerate() {
            let mut ch = c;
            if c == 'S' {
                start = (i, j);
                ch = 'a'; // `S` has the same elevation with `a`
            }
            
            if c == 'E' {
                end = (i, j);
                ch = '{';
            }

            current.push(ch);
        }

        grid.push(current);
    }

    (grid, start, end)
}

fn to_grid_two(input: Vec<String>) -> (Vec<Vec<char> >, Vec<Position>, Position) {
    let mut start: Vec<Position> = vec![];
    let mut end: Position = (0, 0);

    let mut grid: Vec<Vec<char> > = vec![];

    for (i, line) in input.iter().enumerate() {
        let mut current = vec![];

        for (j, c) in line.chars().enumerate() {
            let mut ch = c;
            if c == 'S' || c == 'a' {
                start.push((i, j));
                ch = 'a'; // don't move here
            }
            
            if c == 'E' {
                end = (i, j);
                ch = '{';
            }

            current.push(ch);
        }

        grid.push(current);
    }

    (grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "31");
    }

    #[test]
    fn test_solve_two() {
        let str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "29");
    }
}