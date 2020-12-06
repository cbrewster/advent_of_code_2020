use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    // We need fold_first to be stabilized!
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|a| a.chars().collect::<HashSet<char>>())
                .fold(None, |acc, a| match acc {
                    Some(acc) => Some(&acc & &a),
                    None => Some(a),
                })
                .unwrap_or_default()
                .len()
        })
        .sum()
}
