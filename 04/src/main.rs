use std::collections::HashSet;

const PASSPHRASES: &'static str = include_str!("passphrases.txt");

fn main() {
    println!("Day  4; high-entropy passphrases; First Star: {:?}", first_star());
    println!("Day  4; high-entropy passphrases; Second Star: {:?}", second_star());
}

fn filtered() -> Vec<Vec<&'static str>> {
    PASSPHRASES.lines()
        .map(|passphrase| passphrase.split_whitespace().collect::<Vec<_>>())
        .filter(|words| words.iter().collect::<HashSet<_>>().len() == words.iter().count())
        .collect()
}

fn first_star() -> u32 {
    filtered().len() as u32
}

fn second_star() -> u32 {
    filtered().iter()
        .filter(|words| {
            let palindromic: HashSet<String> = words.iter()
                .map(|word| {
                    let mut sw = word.chars().collect::<Vec<_>>();
                    sw.sort();
                    sw.into_iter().collect()
                }).collect();
            palindromic.len() == words.len()
        })
        .count() as u32
}
