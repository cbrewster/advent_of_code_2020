use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::{Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

fn parse_line(line: &str) -> Result<(&str, Vec<(usize, &str)>)> {
    let mut parts = line.split(" contain ");
    let bag = parts
        .next()
        .context("expected line")?
        .strip_suffix(" bags")
        .context("expected bag name")?;

    let contents = parts.next().context("expected contents")?;
    if contents.contains("no other bags.") {
        return Ok((bag, Vec::new()));
    }

    let contents: Vec<(usize, &str)> = contents
        .split(", ")
        .map(|content| {
            let mut parts = content.splitn(2, " ");
            let count: usize = parts.next().context("expected contents")?.parse()?;
            let bag = parts
                .next()
                .context("expected bag")?
                .trim_end_matches(".")
                .trim_end_matches(" bag")
                .trim_end_matches(" bags");

            Ok((count, bag))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok((bag, contents))
}

fn part1(input: &str) -> Result<usize> {
    let parsed = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    let mut contain_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (bag, contents) in parsed {
        for (_, inner) in &contents {
            contain_map.entry(inner).or_default().insert(bag);
        }
    }

    let mut open_set: Vec<&str> = contain_map
        .get("shiny gold")
        .map(|set| set.iter().cloned().collect())
        .unwrap_or_default();

    let mut closed_set = HashSet::new();
    while let Some(bag) = open_set.pop() {
        if let Some(set) = contain_map.get(bag) {
            open_set.extend(set.iter());
        }
        closed_set.insert(bag);
    }

    Ok(closed_set.len())
}

fn part2(input: &str) -> Result<usize> {
    let parsed = input
        .lines()
        .map(parse_line)
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(get_bag_count(&parsed, "shiny gold"))
}

fn get_bag_count(map: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> usize {
    map.get(bag)
        .iter()
        .map(|l| l.iter())
        .flatten()
        .map(|(count, inner)| count + (count * get_bag_count(map, inner)))
        .sum()
}
