fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();

    let mut x: i32 = 1;
    let mut cycle: u32 = 1;

    let mut total_signal_strength: i32 = 0;

    let mut pixels = vec![false; 240];

    for line in lines.iter() {
        match line.split_once(' ') {
            Some((_addx, v)) => {
                if (cycle + 20) % 40 == 0 {
                    total_signal_strength += cycle as i32 * x;
                }
                if (cycle - 1) % 40 >= (x - 1) as u32 && (cycle - 1) % 40 <= (x + 1) as u32 {
                    pixels[cycle as usize - 1] = true;
                }
                cycle += 1;

                if (cycle + 20) % 40 == 0 {
                    total_signal_strength += cycle as i32 * x;
                }
                if (cycle - 1) % 40 >= (x - 1) as u32 && (cycle - 1) % 40 <= (x + 1) as u32 {
                    pixels[cycle as usize - 1] = true;
                }

                let v = v.parse::<i32>().unwrap();
                x += v;
                cycle += 1;
            }
            None => {
                if (cycle + 20) % 40 == 0 {
                    total_signal_strength += cycle as i32 * x;
                }
                if (cycle - 1) % 40 >= (x - 1) as u32 && (cycle - 1) % 40 <= (x + 1) as u32 {
                    pixels[cycle as usize - 1] = true;
                }
                cycle += 1;
            }
        }
    }

    println!("{}", total_signal_strength);

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", if pixels[40 * i + j] { '#' } else { '.' });
        }
        println!();
    }
}
