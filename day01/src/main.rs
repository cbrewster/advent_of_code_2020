use std::collections::HashSet;
use anyhow::{anyhow, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<isize> {
    let entries: HashSet<isize> = input.split_whitespace().map(|num| num.parse()).collect::<Result<HashSet<_>, _>>()?;
    for entry in entries.iter() {
        if entries.contains(&(2020 - entry)) {
            return Ok(entry * (2020 - entry))
        }
    }
    Err(anyhow!("Could not find answer!"))
}

fn part2(input: &str) -> Result<isize> {
    let entries: HashSet<isize> = input.split_whitespace().map(|num| num.parse()).collect::<Result<HashSet<_>, _>>()?;

    // TODO: Probably could do better than O(n^2) here, but good enough for day 1! :)
    for entry in entries.iter() {
        let target = 2020 - entry;
        if target <= 0 {
            continue;
        }

        for inner_entry in entries.iter() {
            if entries.contains(&(target - inner_entry)) {
                return Ok(entry * inner_entry * (target - inner_entry))
            }
        }

    }
    Err(anyhow!("Could not find answer!"))
}
