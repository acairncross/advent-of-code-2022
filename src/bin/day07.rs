use std::collections::HashMap;

#[derive(Debug)]
enum Tree {
    File(u32),
    Directory(HashMap<String, Tree>),
}

fn add_file(mut tree: &mut Tree, path: &Vec<String>, file: &str, size: u32) {
    for dir in path {
        match tree {
            Tree::File(_) => panic!("oops"),
            Tree::Directory(subtrees) => {
                tree = subtrees.get_mut(dir).unwrap();
            }
        }
    }

    match tree {
        Tree::File(_) => panic!("oops"),
        Tree::Directory(subtrees) => {
            subtrees.insert(file.to_string(), Tree::File(size));
        }
    }
}

fn add_dir(mut tree: &mut Tree, path: &Vec<String>, dir: &str) {
    for dir in path {
        match tree {
            Tree::File(_) => panic!("oops"),
            Tree::Directory(subtrees) => {
                tree = subtrees.get_mut(dir).unwrap();
            }
        }
    }

    match tree {
        Tree::File(_) => panic!("oops"),
        Tree::Directory(subtrees) => {
            subtrees.insert(dir.to_string(), Tree::Directory(HashMap::new()));
        }
    }
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let mut path: Vec<String> = Vec::new();

    let mut tree = Tree::Directory(HashMap::new());

    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.as_slice() {
            ["$", "cd", ".."] => {
                path.pop().unwrap();
            }
            ["$", "cd", dir] => {
                add_dir(&mut tree, &path, dir);
                path.push(dir.to_string());
            }
            ["$", "ls"] => (),
            ["dir", dir] => add_dir(&mut tree, &path, dir),
            [size, file] => add_file(&mut tree, &path, file, size.parse::<u32>().unwrap()),
            _ => panic!("unexpected input"),
        }
    }

    let (total_size, tree_size) = part1(&tree);
    println!("{:?}", total_size);
    println!("{:?}", part2(&tree, tree_size))
}

fn part1(tree: &Tree) -> (u32, u32) {
    let mut total_size = 0;
    let tree_size = part1_rec(tree, &mut total_size);
    (total_size, tree_size)
}

fn part1_rec(tree: &Tree, total_size: &mut u32) -> u32 {
    match tree {
        Tree::File(size) => *size,
        Tree::Directory(subtrees) => {
            let size = subtrees
                .values()
                .map(|subtree| part1_rec(subtree, total_size))
                .sum();
            if size <= 100000 {
                *total_size += size;
            }
            size
        },
    }
}

fn part2_rec(tree: &Tree, required_size: u32, optimal_size: &mut u32) -> u32 {
    match tree {
        Tree::File(size) => *size,
        Tree::Directory(subtrees) => {
            let size = subtrees
                .values()
                .map(|subtree| part2_rec(subtree, required_size, optimal_size))
                .sum();
            if size >= required_size && size < *optimal_size {
                *optimal_size = size
            }
            size
        }
    }
}

fn part2(tree: &Tree, tree_size: u32) -> u32 {
    let required_size = 30000000 - (70000000 - tree_size);
    let mut optimal_size = tree_size; // Initially the whole tree
    part2_rec(tree, required_size, &mut optimal_size);
    optimal_size
}
