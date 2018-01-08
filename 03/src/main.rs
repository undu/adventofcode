const INPUT: u32 = 361527;

fn main() {
    println!("Day 3; spiral memory; First Star: {:?}", first_star());
    println!("Day 3; spiral memory; Second Star: {:?}", second_star());
}

fn first_star() -> u32 {
    spiral_manhattan(INPUT)
}

fn second_star() -> u32 {
    0
}

fn spiral_depth(n: u32) -> u32 {
    if n < 2 {
        0
    }
    else {
        let k = ((n - 1) as f64).sqrt().floor() as u32;
        if k % 2 == 1 {
            (k + 1) / 2
        } else {
            k / 2
        }
    }
}

fn spiral_manhattan(n: u32) -> u32 {
    if n < 2 {
        0
    }
    else {
        let depth = spiral_depth(n);
        let loop_size = depth * 8;
        let loop_start = 2 + (0..depth).map(|k| k * 8).sum::<u32>();
        let loop_walk =
            (0..4).map(|c| loop_start + depth - 1 +
                       ((c * loop_size / 4) as f64).floor() as u32)
            .map(|c| ((n as i64) - (c as i64)).abs() as u32)
            .min().unwrap();
        depth + loop_walk
    }
}
