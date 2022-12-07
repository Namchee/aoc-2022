struct Folder {
    name: String,
    parent: Option<Box<Folder> >,
    children: Vec<Folder>,
    size: u64,
}

impl Folder {
    fn score(&self) -> u64 {
        let mut child_score = 0;
        for child in self.children.iter() {
            child_score += child.score();
        }

        self.size + child_score
    }
}

pub fn solve_one(input: Vec<String>) -> String {
    let root: Folder = Folder {
        name: "/".to_string(),
        parent: None,
        children: vec![],
        size: 0
    };
    let mut cwd: Folder = root;

    let mut read_flag = false;

    for line in input.iter().skip(1) {
        let tokens = line.split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String> >();
    
        if read_flag {
            match tokens[0].as_str() {
                "$" => read_flag = false,
                _ => {
                    if tokens[0] == "dir" {
                        cwd.children.push(Folder{
                            name: tokens[1].clone(),
                            parent: Option::from(Box::from(cwd)),
                            children: vec![],
                            size: 0,
                        });
                    } else {
                        cwd.size += tokens[0].parse::<u64>().unwrap();
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
                cwd = *cwd.parent.unwrap();
            } else {
                cwd = *cwd.children.iter().find(|x| x.name == tokens[2]).unwrap();
            }
        }
    }

    format!("{}", get_scores(&root))
}

pub fn solve_two(input: Vec<String>) -> String {
    "".to_string()
}

fn get_scores(current: &Folder) -> u64 {
    let mut score: u64 = 0;
    if current.score() < 100000 {
        score += current.score();
    }

    for child in current.children.iter() {
        score += get_scores(child);
    }

    score
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
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "");
    }
}