fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let elf_calories: Vec<u32> = lines
        .split(|s| s.is_empty())
        .map(|elf_items| {
            elf_items
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    println!("{}", part1(&elf_calories));
    println!("{}", part2(&elf_calories));
}

fn part1(elf_calories: &Vec<u32>) -> u32 {
    elf_calories.iter().max().unwrap().clone()
}

fn part2(elf_calories: &Vec<u32>) -> u32 {
    let mut elf_calories = elf_calories.clone();
    elf_calories.sort();
    elf_calories.reverse();
    elf_calories.iter().take(3).sum()
}
