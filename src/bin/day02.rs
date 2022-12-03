fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let rounds: Vec<(u32, u32)> = lines
        .iter()
        .map(|line| {
            match line
                .split(" ")
                .map(|s| parse(s))
                .collect::<Vec<u32>>()
                .as_slice()
            {
                [them, you] => (them.clone(), you.clone()),
                _ => panic!("expected 2 elements"),
            }
        })
        .collect();

    let score1: u32 = rounds
        .iter()
        .map(|(them, you)| shape_score(&you) + result_score(&them, &you))
        .sum();

    let score2: u32 = rounds
        .iter()
        .map(|(them, result)| {
            let you = to_shape(&them, &result);
            shape_score(&you) + result_score(&them, &you)
        })
        .sum();

    println!("{}", score1);
    println!("{}", score2);
}

fn parse(s: &str) -> u32 {
    match s {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("unexpected character"),
    }
}

fn shape_score(shape: &u32) -> u32 {
    shape + 1
}

fn result_score(them: &u32, you: &u32) -> u32 {
    3 * to_result(&them, &you)
}

fn to_result(them: &u32, you: &u32) -> u32 {
    (3 + you - them + 1) % 3
}

fn to_shape(them: &u32, result: &u32) -> u32 {
    (3 + result + them - 1) % 3
}
