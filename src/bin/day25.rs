fn main() {
    let snafus: Vec<Vec<i64>> = std::io::stdin()
        .lines()
        .map(|line| parse_digits(&line.unwrap()))
        .collect();

    println!("{}", part1(&snafus));
}

fn part1(snafus: &[Vec<i64>]) -> String {
    let sum: i64 = snafus.iter().map(|snafu| eval_snafu(&snafu)).sum();
    uneval_snafu(sum)
        .iter()
        .map(|digit| unparse_digit(*digit))
        .collect::<String>()
}

fn parse_digits(line: &str) -> Vec<i64> {
    line.chars().map(&parse_digit).collect()
}

fn parse_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("unexpected input"),
    }
}

fn eval_snafu(snafu: &[i64]) -> i64 {
    snafu
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * i64::pow(5, i as u32))
}

fn uneval_snafu(decimal: i64) -> Vec<i64> {
    let mut decimal = decimal;
    let mut snafu: Vec<i64> = vec![];
    while decimal != 0 {
        let rem = decimal % 5;
        snafu.push(if rem > 2 { rem - 5 } else { rem });
        decimal = (decimal + 2) / 5;
    }
    snafu.iter().copied().rev().collect()
}

fn unparse_digit(digit: i64) -> char {
    match digit {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("unexpected input"),
    }
}
