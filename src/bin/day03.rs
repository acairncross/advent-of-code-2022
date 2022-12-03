use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines))
}

fn part1(lines: &Vec<String>) -> u32 {
    let items = lines.iter().map(|line| {
        let left: HashSet<char> = line[..line.len() / 2].chars().collect();
        let right: HashSet<char> = line[line.len() / 2..].chars().collect();
        let intersection = &left & &right;
        assert!(intersection.len() == 1);
        let item = intersection.into_iter().collect::<Vec<char>>()[0];
        item
    });

    items.map(&priority).sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    let badges = lines.chunks(3).map(|packs| {
        let mut pack_intersection: HashSet<char> = packs[0].chars().collect();
        for pack in packs {
            let pack: HashSet<char> = pack.chars().collect();
            pack_intersection = &pack_intersection & &pack;
        }
        assert!(pack_intersection.len() == 1);
        let badge = pack_intersection.into_iter().collect::<Vec<char>>()[0];
        badge
    });

    badges.map(&priority).sum()
}

fn priority(item: char) -> u32 {
    if item >= 'a' {
        1 + item as u32 - 'a' as u32
    } else {
        1 + 26 + item as u32 - 'A' as u32
    }
}
