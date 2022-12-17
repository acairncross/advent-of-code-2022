use std::collections::HashMap;

type Shape = Vec<Vec<bool>>;

type Pos = (usize, usize);
type Skyline = [usize; 7];
type State = (Pos, usize, usize, Skyline);

fn main() {
    let shapes: Vec<Shape> = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        // "Upside down" (row 0 is the bottom row)
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let jet_pattern: Vec<char> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();

    println!("{}", simulate(&jet_pattern, &shapes, 2022));
    println!("{}", simulate(&jet_pattern, &shapes, 1000000000000));
}

fn simulate(jet_pattern: &[char], shapes: &[Shape], max_rocks: u64) -> u64 {
    let mut chamber = vec![vec![false; 7]; 4];

    let mut x = 2;
    let mut y = chamber.len() - 1;

    let mut num_rocks = 0;
    let mut num_steps = 0;

    let mut visited: HashMap<State, (u64, u64, u64)> = HashMap::new();

    let mut implicit_height = 0;

    while num_rocks < max_rocks {
        let dir = jet_pattern[num_steps % jet_pattern.len()];
        let shape_index = num_rocks as usize % shapes.len();

        if dir == '<' && x > 0 && !has_collision(&shapes[shape_index], x - 1, y, &chamber) {
            x -= 1;
        }

        if dir == '>'
            && x + shapes[shape_index][0].len() < 7
            && !has_collision(&shapes[shape_index], x + 1, y, &chamber)
        {
            x += 1;
        }

        if y > 0 && !has_collision(&shapes[shape_index], x, y - 1, &chamber) {
            y -= 1;
        } else {
            insert_shape(&shapes[shape_index], x, y, &mut chamber);

            x = 2;
            y = chamber.len() - 1;
            num_rocks += 1;
        }

        num_steps += 1;

        let skyline = chamber_to_skyline(&chamber);
        let state = (
            (x, chamber.len() - y),
            num_steps % jet_pattern.len(),
            num_rocks as usize % shapes.len(),
            skyline,
        );
        if implicit_height == 0 {
            match visited.get(&state) {
                Some((prev_steps, prev_rocks, prev_height)) => {
                    let cycle_length_steps = num_steps as u64 - prev_steps;
                    let cycle_length_rocks = num_rocks - prev_rocks;
                    let cycle_length_height = tower_height(&chamber) as u64 - prev_height;

                    let num_rocks_remaining = max_rocks - num_rocks;
                    let num_cycles_remaining = num_rocks_remaining / cycle_length_rocks;

                    if num_cycles_remaining > 0 {
                        num_steps += (num_cycles_remaining * cycle_length_steps) as usize;
                        num_rocks += num_cycles_remaining * cycle_length_rocks;
                        implicit_height += num_cycles_remaining * cycle_length_height;
                    }
                }
                None => {
                    visited.insert(
                        state,
                        (num_steps as u64, num_rocks, tower_height(&chamber) as u64),
                    );
                }
            }
        }
    }

    tower_height(&chamber) as u64 + implicit_height
}

fn has_collision(shape: &Shape, x: usize, y: usize, chamber: &[Vec<bool>]) -> bool {
    for j in 0..shape.len() {
        if y + j >= chamber.len() {
            continue;
        }
        for i in 0..shape[0].len() {
            if shape[j][i] && chamber[y + j][x + i] {
                return true;
            }
        }
    }
    false
}

fn insert_shape(shape: &Shape, x: usize, y: usize, chamber: &mut Vec<Vec<bool>>) {
    for j in 0..shape.len() {
        for i in 0..shape[0].len() {
            chamber[y + j][x + i] |= shape[j][i];
        }
    }

    let height = tower_height(chamber);
    while chamber.len() < height + 4 {
        chamber.push(vec![false; 7]);
    }
}

#[allow(dead_code)]
fn print_chamber(chamber: &[Vec<bool>]) {
    for row in chamber.iter().rev() {
        for x in row {
            if *x {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn tower_height(chamber: &[Vec<bool>]) -> usize {
    for j in (0..chamber.len()).rev() {
        for i in 0..chamber[0].len() {
            if chamber[j][i] {
                return j + 1;
            }
        }
    }
    0
}

fn chamber_to_skyline(chamber: &[Vec<bool>]) -> Skyline {
    let mut skyline = [0; 7];
    for j in (0..chamber.len()).rev() {
        if skyline.iter().all(|x| x > &0) {
            break;
        }
        for i in 0..chamber[0].len() {
            if chamber[j][i] {
                skyline[i] = chamber.len() - j;
            }
        }
    }
    skyline
}
