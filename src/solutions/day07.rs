use std::collections::HashMap;
use std::cmp::min;

pub fn solve_one(input: Vec<String>) -> String {
    let (folders, _) = get_folder_map(input);

    let mut count: u64 = 0;
    for (_, v) in folders.iter() {
        if v <= &100000 {
            count += v;
        }
    }

    format!("{}", count)
}

pub fn solve_two(input: Vec<String>) -> String {
    let (folders, used) = get_folder_map(input);
    let unused = 70000000 - used;
    let required = 30000000 - unused;

    let mut best: u64 = folders[&"root".to_string()];
    for (_, sz) in folders.iter() {
        if sz >= &required {
            best = min(best, *sz);
        }
    }

    format!("{}", best)
}

fn get_folder_map(input: Vec<String>) -> (HashMap<String, u64>, u64) {
    let mut folders: HashMap<String, u64> = HashMap::from([
        ("root".to_string(), 0),
    ]);
    let mut current: String = "root".to_string();
    let mut read_flag = false;

    let mut used: u64 = 0;

    for line in input.iter().skip(1) {
        let tokens = line.split(" ")
            .collect::<Vec<_> >();
    
        if read_flag {
            match tokens[0] {
                "$" => read_flag = false,
                _ => {
                    if tokens[0] == "dir" {
                        let dir = format!(
                            "{}/{}",
                            current,
                            tokens[1],
                        );

                        folders.insert(dir, 0);
                    } else {
                        let sz = tokens[0].parse::<u64>().unwrap();
                        used += sz;

                        let mut section = current.split("/")
                            .collect::<Vec<_> >();
                        while section.len() > 0 {
                            let path = section.join("/");
                            let val = folders[&path];

                            folders.insert(path, val + sz);

                            section.pop();
                        }
                    }
                },
            }
        }

        if !read_flag {
            if tokens[1] == "ls" {
                read_flag = true;
                continue;
            }

            if tokens[2] == ".." {
                let mut paths = current.split("/").collect::<Vec<_> >();
                paths.pop();

                current = paths.join("/");
            } else {
                current = format!("{}/{}", current, tokens[2]);
            }
        }
    }

    (folders, used)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "95437");
    }

    #[test]
    fn test_solve_two() {
        let str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();
    
        assert_eq!(solve_two(input), "24933642");
    }
}