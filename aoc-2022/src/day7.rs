use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirNode {
    Directory(usize),                   // Index of child directory
    File { name: String, size: usize }, // File info
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Directory {
    parent: usize, // index of parent directory
    name: String,
    size: usize,            // combined size of all children
    children: Vec<DirNode>, // vec of all child nodes
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Directory> {
    let mut filesystem = vec![Directory {
        parent: 0,
        name: "/".to_owned(),
        size: 0,
        children: Vec::new(),
    }]; // Start with an empty root directory
    let mut current_dir = 0;

    for line in input.lines() {
        if line.starts_with("$ cd /") {
            // Change directory command to root
            current_dir = 0;
            //println!("$ cd /");
        } else if line.starts_with("$ cd ..") {
            // Change directory command to parent
            current_dir = filesystem[current_dir].parent;
            //println!("$ cd .. [{}]", filesystem[current_dir].name);
        } else if line.starts_with("$ cd ") {
            // Change directory command (standard)
            let mut next_dir = None;
            let search_name = line.split_at(5).1;

            for child in filesystem[current_dir].children.iter() {
                match child {
                    DirNode::Directory(child_index) => {
                        if filesystem[*child_index].name == search_name {
                            next_dir = Some(*child_index);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            current_dir = next_dir.expect("Tried to change to invalid directory");
            //println!("$ cd [{}]", filesystem[current_dir].name);
        } else if line.starts_with("$ ls") {
            // List directory command
            // Do nothing, parse the following lines
            //println!("$ ls");
        } else if line.starts_with("dir ") {
            // New directory
            let name = line.split_at(4).1.to_owned();
            //println!("dir [{}] in [{}]", name, filesystem[current_dir].name);

            filesystem.push(Directory {
                parent: current_dir,
                name,
                size: 0,
                children: Vec::new(),
            });
            let new_index = filesystem.len() - 1;
            filesystem[current_dir]
                .children
                .push(DirNode::Directory(new_index));
        } else {
            // New file
            let mut parts = line.split(' ');
            let size = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap().to_owned();
            //println!("file {} [{}] in [{}]", size, name, filesystem[current_dir].name);


            filesystem[current_dir]
                .children
                .push(DirNode::File { name, size });

            let mut parent_dir = current_dir;

            loop {
                filesystem[parent_dir].size += size;

                if parent_dir == 0 {
                    // Just added size to root
                    break;
                } else {
                    parent_dir = filesystem[parent_dir].parent;
                }
            }
        }
    }

    filesystem
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Directory]) -> usize {
    input
        .iter()
        .map(|dir| dir.size)
        .filter(|size| *size <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Directory]) -> usize {
    let used_space = input[0].size;
    let disk_space = 70_000_000;
    let min_free = 30_000_000;
    let max_used = disk_space - min_free;
    let must_delete = used_space - max_used; // assume input is valid and too much space is used

    // println!("Directory sizes: {:?}", input.iter().map(|dir| dir.size).collect::<Vec<_>>());
    // println!("Used space: {}", used_space);
    // println!("Must delete: {}", must_delete);

    input
        .iter()
        .map(|dir| dir.size)
        .filter(|size| *size >= must_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let example = r#"$ cd /
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
7214296 k"#;
        let output = input_generator(example);

        let expected = vec![
            Directory { // 0
                parent: 0,
                name: "/".to_owned(),
                size: 48381165,
                children: vec![
                    DirNode::Directory(
                        1, // a
                    ),
                    DirNode::File {
                        name: "b.txt".to_owned(),
                        size: 14848514,
                    },
                    DirNode::File {
                        name: "c.dat".to_owned(),
                        size: 8504156,
                    },
                    DirNode::Directory(
                        2, // d
                    ),
                ],
            },
            Directory { // 1
                parent: 0,
                name: "a".to_owned(),
                size: 94853,
                children: vec![
                    DirNode::Directory(
                        3, // e
                    ),
                    DirNode::File {
                        name: "f".to_owned(),
                        size: 29116,
                    },
                    DirNode::File {
                        name: "g".to_owned(),
                        size: 2557,
                    },
                    DirNode::File {
                        name: "h.lst".to_owned(),
                        size: 62596,
                    },
                ],
            },
            Directory { // 2
                parent: 0,
                name: "d".to_owned(),
                size: 24933642,
                children: vec![
                    DirNode::File {
                        name: "j".to_owned(),
                        size: 4060174,
                    },
                    DirNode::File {
                        name: "d.log".to_owned(),
                        size: 8033020,
                    },
                    DirNode::File {
                        name: "d.ext".to_owned(),
                        size: 5626152,
                    },
                    DirNode::File {
                        name: "k".to_owned(),
                        size: 7214296,
                    },
                ],
            },
            Directory { // 3
                parent: 1,
                name: "e".to_owned(),
                size: 584,
                children: vec![
                    DirNode::File {
                        name: "i".to_owned(),
                        size: 584,
                    },
                ],
            },
        ];

        println!("Example output:\n{:#?}", output);

        assert_eq!(output, expected);
    }
}
