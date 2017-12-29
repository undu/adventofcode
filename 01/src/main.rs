const CAPTCHA: &'static str = include_str!("inverse_captcha.txt");

fn main() {
    println!("Day 1; inverse captcha; First Star: {:?}", first_star());
    println!("Day 1; inverse captcha; Second Star: {:?}", second_star());
}

fn first_star() -> u32 {
    captcha(1)
}

fn second_star() -> u32 {
    captcha(CAPTCHA.len() / 2)
}

fn captcha(skipper: usize) -> u32 {
    CAPTCHA.chars()
        .zip(CAPTCHA.chars().cycle().skip(skipper))
        .filter(|&(one, consec)| one.is_digit(10) && one == consec)
        .map(|(one, _)| one.to_digit(10).expect("Impossibru"))
        .sum()
}
