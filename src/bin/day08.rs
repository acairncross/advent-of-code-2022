fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());

    let grid: Vec<Vec<u32>> = lines
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{:?}", part1(&grid));
    println!("{:?}", part2(&grid))
}

fn part1(grid: &Vec<Vec<u32>>) -> u32 {
    let visible_left = make_visibility_grid(grid, false, false);
    let visible_right = make_visibility_grid(grid, true, false);
    let visible_top = make_visibility_grid(grid, false, true);
    let visible_bottom = make_visibility_grid(grid, true, true);

    let size = grid.len();
    let mut visible = vec![vec![false; size]; size];
    for i in 0..size {
        for j in 0..size {
            visible[i][j] = visible_left[i][j]
                || visible_right[i][j]
                || visible_top[i][j]
                || visible_bottom[i][j];
        }
    }

    visible
        .iter()
        .map(|row| row.iter().map(|b| *b as u32).sum::<u32>())
        .sum()
}

fn part2(grid: &Vec<Vec<u32>>) -> u32 {
    let scenic_score_left = make_scenic_score_grid(grid, false, false);
    let scenic_score_right = make_scenic_score_grid(grid, true, false);
    let scenic_score_top = make_scenic_score_grid(grid, false, true);
    let scenic_score_bottom = make_scenic_score_grid(grid, true, true);

    let size = grid.len();
    let mut scenic_score = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            scenic_score[i][j] = scenic_score_left[i][j]
                * scenic_score_right[i][j]
                * scenic_score_top[i][j]
                * scenic_score_bottom[i][j];
        }
    }

    *scenic_score
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

fn make_visibility_grid(grid: &Vec<Vec<u32>>, reverse: bool, vertical: bool) -> Vec<Vec<bool>> {
    let size = grid.len();
    let mut visible_grid = vec![vec![true; size]; size];
    for i in 0..size {
        let mut max = 0;
        let mut range: Box<dyn Iterator<Item = usize>> = Box::new(0..size);
        let mut range_rev: Box<dyn Iterator<Item = usize>> = Box::new((0..size).rev());
        for j in if reverse { &mut range_rev } else { &mut range } {
            if vertical {
                visible_grid[j][i] = grid[j][i] > max || j == if reverse { size - 1 } else { 0 };
                max = std::cmp::max(max, grid[j][i]);
            } else {
                visible_grid[i][j] = grid[i][j] > max || j == if reverse { size - 1 } else { 0 };
                max = std::cmp::max(max, grid[i][j]);
            }
        }
    }

    visible_grid
}

fn make_scenic_score_grid(grid: &Vec<Vec<u32>>, reverse: bool, vertical: bool) -> Vec<Vec<u32>> {
    let size = grid.len();
    let mut scenic_score_grid = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            let pivot = if vertical { i } else { j };
            let mut range: Box<dyn Iterator<Item = usize>> = Box::new((0..pivot).rev());
            let mut range_rev: Box<dyn Iterator<Item = usize>> = Box::new(pivot + 1..size);
            for k in if reverse { &mut range_rev } else { &mut range } {
                scenic_score_grid[i][j] += 1;
                let tree = if vertical { grid[k][j] } else { grid[i][k] };
                if tree >= grid[i][j] {
                    break;
                }
            }
        }
    }

    scenic_score_grid
}
