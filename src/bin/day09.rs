use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    solve(lines, 2)
}

fn part2(lines: &Vec<String>) -> usize {
    solve(lines, 10)
}

fn solve(lines: &Vec<String>, num_knots: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); num_knots];

    let mut visited_sets: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(); knots.len()];
    for (i, visited_set) in visited_sets.iter_mut().enumerate() {
        visited_set.insert(knots[i]);
    }

    for line in lines.iter() {
        match line.split_once(' ') {
            Some((direction, count)) => {
                let count = count.parse::<i32>().unwrap();
                for _ in 0..count {
                    match direction {
                        "R" => {
                            knots[0].0 += 1;
                        }
                        "L" => {
                            knots[0].0 -= 1;
                        }
                        "U" => {
                            knots[0].1 += 1;
                        }
                        "D" => {
                            knots[0].1 -= 1;
                        }
                        _ => panic!("unexpected direction"),
                    }
                    for i in 1..num_knots {
                        if knots[i] == (knots[i - 1].0 + 2, knots[i - 1].1) {
                            knots[i].0 -= 1;
                        } else if knots[i] == (knots[i - 1].0 - 2, knots[i - 1].1) {
                            knots[i].0 += 1;
                        } else if knots[i] == (knots[i - 1].0, knots[i - 1].1 + 2) {
                            knots[i].1 -= 1;
                        } else if knots[i] == (knots[i - 1].0, knots[i - 1].1 - 2) {
                            knots[i].1 += 1;
                        } else if !touching(knots[i - 1], knots[i]) {
                            knots[i].0 += i32::signum(knots[i - 1].0 - knots[i].0);
                            knots[i].1 += i32::signum(knots[i - 1].1 - knots[i].1);
                        }
                        visited_sets[i].insert(knots[i]);
                    }
                }
            }
            None => panic!("unexpected input"),
        };
    }

    visited_sets[num_knots - 1].len()
}

fn touching(x: (i32, i32), y: (i32, i32)) -> bool {
    i32::abs(x.0 - y.0) <= 1 && i32::abs(x.1 - y.1) <= 1
}
