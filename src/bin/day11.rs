use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    rule: Rule,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Option<u64>,
    right: Option<u64>,
}

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
struct Rule {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();
    let stanzas: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();

    println!("{}", part1(&stanzas));
    println!("{}", part2(&stanzas))
}

fn part1(stanzas: &Vec<&[String]>) -> u64 {
    solve(stanzas, false)
}

fn part2(stanzas: &Vec<&[String]>) -> u64 {
    solve(stanzas, true)
}

fn solve(stanzas: &Vec<&[String]>, part2: bool) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for stanza in stanzas {
        let mut stanza_iter = stanza.iter();

        stanza_iter.next();

        let items = VecDeque::from(
            stanza_iter
                .next()
                .map(|line| extract_integers(line))
                .unwrap(),
        );

        let operation = stanza_iter
            .next()
            .map(|line| extract_operation(line))
            .unwrap();

        let divisible_by = stanza_iter
            .next()
            .map(|line| extract_integer::<u64>(line))
            .unwrap();

        let if_true = stanza_iter
            .next()
            .map(|line| extract_integer::<usize>(line))
            .unwrap();

        let if_false = stanza_iter
            .next()
            .map(|line| extract_integer::<usize>(line))
            .unwrap();

        let rule = Rule {
            divisible_by,
            if_true,
            if_false,
        };

        monkeys.push(Monkey {
            items,
            operation,
            rule,
        });
    }

    let mut num_inspections: Vec<u64> = vec![0; monkeys.len()];

    let multiples = monkeys.iter().map(|monkey| monkey.rule.divisible_by);
    let mut multiple = 1;
    for each_multiple in multiples {
        multiple *= each_multiple;
    }

    for _round in 0..(if part2 { 10000 } else { 20 }) {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = &mut monkeys[monkey_index];
                let item = match monkey.items.pop_front() {
                    Some(item) => item,
                    None => continue,
                };
                let left = monkey.operation.left.unwrap_or(item);
                let right = monkey.operation.right.unwrap_or(item);
                let new_item = match monkey.operation.operator {
                    Operator::Plus => left + right,
                    Operator::Times => left * right,
                };
                let new_item = if part2 {
                    new_item % multiple
                } else {
                    new_item / 3
                };
                let if_true = monkey.rule.if_true;
                let if_false = monkey.rule.if_false;
                if new_item % monkey.rule.divisible_by == 0 {
                    monkeys[if_true].items.push_back(new_item);
                } else {
                    monkeys[if_false].items.push_back(new_item);
                }
                num_inspections[monkey_index] += 1;
            }
        }
    }

    num_inspections.sort();
    num_inspections[num_inspections.len() - 1] * num_inspections[num_inspections.len() - 2]
}

fn extract_integer<F: std::str::FromStr + Copy>(line: &str) -> F {
    let xs = line
        .split(' ')
        .filter_map(|integer| integer.parse::<F>().ok())
        .collect::<Vec<_>>();

    match xs.as_slice() {
        [x] => *x,
        _ => panic!("unexpected number of values in line"),
    }
}

fn extract_integers(line: &str) -> Vec<u64> {
    match line.split_once(": ") {
        Some((_, integers)) => integers
            .split(", ")
            .map(|integer| integer.parse::<u64>().unwrap())
            .collect(),
        None => panic!("unexpected input"),
    }
}

fn extract_operation(line: &str) -> Operation {
    match line.split_once("= ") {
        Some((_, expression)) => match expression.split(' ').collect::<Vec<_>>().as_slice() {
            [left_str, operator_str, right_str] => {
                let left = left_str.parse::<u64>().ok();
                let operator = match *operator_str {
                    "+" => Operator::Plus,
                    "*" => Operator::Times,
                    _ => panic!("unexpected input"),
                };
                let right = right_str.parse::<u64>().ok();

                Operation {
                    operator,
                    left,
                    right,
                }
            }
            _ => panic!("expected 3 tokens"),
        },
        None => panic!("unexpected input"),
    }
}
