const SPREADSHEET: &'static str = include_str!("spreadsheet.txt");

fn main() {
    println!("Day  2; corruption checksum; First Star: {:?}", first_star());
    println!("Day  2; corruption checksum; Second Star: {:?}", second_star());
}

fn first_star() -> u32 {
    SPREADSHEET.lines()
        .map(|row| row.split_whitespace()
             .map(|num| num.parse().unwrap()).collect::<Vec<u32>>())
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn second_star() -> u32 {
    SPREADSHEET.lines()
        .map(|row| row.split_whitespace()
             .map(|num| num.parse().unwrap()).collect::<Vec<u32>>())
        .map(|row| {
            for n in &row {
                for m in &row {
                    if n % m == 0 && n != m {
                        return n / m;
                    }
                }
            }
            0
        })
        .sum()
}
