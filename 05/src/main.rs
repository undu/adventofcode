const JUMPS: &'static str = include_str!("jumps.txt");

fn main() {
    println!("Day  5; A maze of twisty trampolines, all alike; First Star: {:?}", first_star());
    println!("Day  5; A maze of twisty trampolines, all alike; Second Star: {:?}", second_star());
}

fn jumps() -> Vec<i32> {
    JUMPS.lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn trampolines(threshold: i32) -> u32 {
    let mut jumps = jumps();
    let mut jumped = 0;
    let mut next = 0;
    while next < jumps.len() {
        let jump = jumps[next] as usize;
        if threshold != 0 && jumps[next] >= threshold {
            jumps[next] -= 1
        } else {
            jumps[next] += 1;
        }
        next += jump;
        jumped += 1;
    }
    jumped
}

fn first_star() -> u32 {
    trampolines(0)
}
fn second_star() -> u32 {
    trampolines(3)
}
