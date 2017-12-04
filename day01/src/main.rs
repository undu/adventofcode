const CAPTCHA: &'static str = include_str!("inverse_captcha.txt");

fn main() {
    println!("Day 1; inverse captcha; First Star: {:?}", first_star());
    println!("Day 1; inverse captcha; Second Star: {:?}", second_star());
}

fn first_star() -> u32 {
    CAPTCHA.chars()
        .zip(CAPTCHA.chars().cycle().skip(1))
        .filter(|&(one, consec)| one.is_digit(10) && one == consec)
        .map(|(one, _)| one.to_digit(10).expect("Impossibru"))
        .sum()
}

fn second_star() -> u32 {
    CAPTCHA.chars()
        .zip(CAPTCHA.chars().cycle().skip(CAPTCHA.len() / 2))
        .filter(|&(one, consec)| one.is_digit(10) && one == consec)
        .map(|(one, _)| one.to_digit(10).expect("Impossibru"))
        .sum()
}
