use anyhow::{anyhow, Result, Context};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

// Example Input
// 4-5 q: cwmhd

struct Policy {
    min: usize,
    max: usize,
    c: char,
}

struct Entry<'a> {
    policy: Policy,
    password: &'a str
}

impl Entry<'_> {
    fn is_valid_sled_rental(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.policy.c).count();
        count >= self.policy.min && count <= self.policy.max
    }

    fn is_valid_toboggan(&self) -> Result<bool> {
        let a = self.password.chars().nth(self.policy.min - 1).context("invalid position")?;
        let b = self.password.chars().nth(self.policy.max - 1).context("invalid position")?;
        Ok((a == self.policy.c) ^ (b == self.policy.c))
    }
}

fn parse_entry(entry: &str) -> Result<Entry> {
    let mut parts = entry.split(":");
    let policy = parts.next().context("could not get policy")?;
    let password = parts.next().context("could not get policy")?.trim();

    let mut policy_parts = policy.split_whitespace();
    let range = policy_parts.next().context("could not get policy range")?;
    let c = policy_parts.next().context("could not get policy char")?;
    if c.chars().count() != 1 {
        return Err(anyhow!("policy char count invalid, must be single char"));
    }
    let c = c.chars().next().unwrap();

    let mut range_parts = range.split("-");
    let min = range_parts.next().context("failed to get min on range")?.parse()?;
    let max = range_parts.next().context("failed to get max on range")?.parse()?;

    let policy = Policy {
        min,
        max,
        c
    };

    Ok(Entry {
        policy,
        password
    })
}

fn parse_entries(entries: &str) -> Result<Vec<Entry>> {
    entries.lines().map(parse_entry).collect()
}

fn part1(input: &str) -> Result<usize> {
    let entries = parse_entries(input)?;
    let valid_count = entries.iter().filter(|e| e.is_valid_sled_rental()).count();

    Ok(valid_count)
}

fn part2(input: &str) -> Result<usize> {
    let entries = parse_entries(input)?;
    let valid_count = entries.iter().filter(|e| e.is_valid_toboggan().unwrap()).count();

    Ok(valid_count)
}
