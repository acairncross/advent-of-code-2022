use std::collections::HashSet;

fn main() {
    let line: String = std::io::stdin().lines().next().unwrap().unwrap();
    let datastream: Vec<char> = line.chars().collect();

    println!("{}", part1(&datastream));
    println!("{}", part2(&datastream));
}

fn part1(datastream: &Vec<char>) -> u32 {
    find_marker(datastream, 4) as u32
}

fn part2(datastream: &Vec<char>) -> u32 {
    find_marker(datastream, 14) as u32
}

fn find_marker(datastream: &Vec<char>, marker_length: usize) -> usize {
    let valid_marker = datastream
        .windows(marker_length as usize)
        .map(|window| window.iter().cloned().collect::<HashSet<char>>().len() == window.len());
    let processed_chars: Vec<bool> = valid_marker.take_while(|valid| !valid).collect();
    processed_chars.len() + marker_length
}
