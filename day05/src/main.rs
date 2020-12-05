use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

fn parse_seat(seat: &str) -> Result<usize> {
    if seat.len() != 10 {
        return Err(anyhow!("seat must be 10 long"));
    }

    // TODO: This doesn't verify that the first 7 are F/B and last 3 are L/R
    let mut id = 0;
    for c in seat.chars() {
        match c {
            'F' | 'L' => {
                id = id << 1;
            }
            'B' | 'R' => id = (id << 1) + 1,
            _ => return Err(anyhow!("invalid seat")),
        }
    }

    Ok(id)
}

fn part1(input: &str) -> Result<usize> {
    let ids = input.lines().map(parse_seat).collect::<Result<Vec<_>>>()?;
    ids.iter().max().cloned().context("no ids")
}

fn part2(input: &str) -> Result<usize> {
    let ids = input
        .lines()
        .map(parse_seat)
        .collect::<Result<HashSet<_>>>()?;

    // TODO: We could probably write loop that adds to the hashset and computes these at the same
    // time.
    let min_id = *ids.iter().min().context("no ids")?;
    let max_id = *ids.iter().max().context("no ids")?;

    for id in min_id..max_id {
        if !ids.contains(&id) {
            return Ok(id);
        }
    }
    Err(anyhow!("could not find id"))
}

#[test]
fn test_parse_seat() -> Result<()> {
    assert_eq!(357, parse_seat("FBFBBFFRLR")?);
    Ok(())
}
