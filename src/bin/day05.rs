fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let mut lines_iter = lines.iter();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines_iter.next() {
        if stacks.len() == 0 {
            for _ in 0..((line.len() + 1) / 4) {
                stacks.push(Vec::new());
            }
        }

        if !line.contains("[") {
            break;
        }

        let line: Vec<char> = line.chars().collect();
        for i in 0..((line.len() + 1) / 4) {
            if line[4 * i + 1] != ' ' {
                stacks[i].push(line[4 * i + 1]);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    _ = lines_iter.next();

    let instrs: Vec<(usize, usize, usize)> = lines_iter
        .map(|line| {
            let count_from_to: Vec<usize> = line
                .split(" ")
                .filter(|token| str_is_numeric(token))
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            match count_from_to.as_slice() {
                [count, from, to] => (*count, *from, *to),
                _ => panic!("expected 3 elements"),
            }
        })
        .collect();

    println!("{}", part1(stacks.clone(), &instrs));
    println!("{}", part2(stacks.clone(), &instrs))
}

fn part1(mut stacks: Vec<Vec<char>>, instrs: &Vec<(usize, usize, usize)>) -> String {
    for (count, from, to) in instrs.iter() {
        for _ in 0..*count {
            if let Some(x) = stacks[from - 1].pop() {
                stacks[to - 1].push(x)
            } else {
                panic!("expected to be able to pop")
            }
        }
    }
    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

fn part2(mut stacks: Vec<Vec<char>>, instrs: &Vec<(usize, usize, usize)>) -> String {
    for (count, from, to) in instrs.iter() {
        let mut multi_crate: Vec<char> = Vec::new();
        for _ in 0..*count {
            if let Some(x) = stacks[from - 1].pop() {
                multi_crate.push(x)
            } else {
                panic!("expected to be able to pop");
            }
        }
        multi_crate.reverse();
        stacks[to - 1].extend_from_slice(&multi_crate);
    }
    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

fn str_is_numeric(s: &str) -> bool {
    s.chars()
        .map(|c| char::is_numeric(c))
        .fold(true, |acc, x| acc && x)
}
