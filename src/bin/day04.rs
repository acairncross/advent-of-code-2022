#[derive(Debug, Clone)]
struct InclusiveRange {
    start: u32,
    end: u32,
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let section_pairs: Vec<(InclusiveRange, InclusiveRange)> = lines
        .iter()
        .map(|line| {
            let ranges = line
                .split(",")
                .map(|range| {
                    let sections = range
                        .split("-")
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    match sections.as_slice() {
                        [lo, hi] => InclusiveRange {
                            start: lo.clone(),
                            end: hi.clone(),
                        },
                        _ => panic!("expected 2 elements"),
                    }
                })
                .collect::<Vec<InclusiveRange>>();
            match ranges.as_slice() {
                [fst, snd] => (fst.clone(), snd.clone()),
                _ => panic!("expected 2 elements"),
            }
        })
        .collect();

    println!("{}", part1(&section_pairs));
    println!("{}", part2(&section_pairs))
}

fn part1(section_pairs: &Vec<(InclusiveRange, InclusiveRange)>) -> u32 {
    section_pairs
        .iter()
        .map(|(a, b)| if either_fully_contains(a, b) { 1 } else { 0 })
        .sum()
}

fn part2(section_pairs: &Vec<(InclusiveRange, InclusiveRange)>) -> u32 {
    section_pairs
        .iter()
        .map(|(a, b)| if overlaps(a, b) { 1 } else { 0 })
        .sum()
}

fn either_fully_contains(a: &InclusiveRange, b: &InclusiveRange) -> bool {
    fully_contains(a, b) || fully_contains(b, a)
}

fn fully_contains(a: &InclusiveRange, b: &InclusiveRange) -> bool {
    a.start <= b.start && a.end >= b.end
}

fn overlaps(a: &InclusiveRange, b: &InclusiveRange) -> bool {
    !(a.end < b.start || a.start > b.end)
}
