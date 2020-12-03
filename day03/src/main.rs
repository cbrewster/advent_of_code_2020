use std::convert::TryFrom;

use anyhow::{anyhow, Result, Context};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Tree),
            _ => Err(anyhow!("unknown tile {}", value))
        }
    }
}

struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(input: &str) -> Result<Map> {
        let tiles = input.lines().map(|line| {
            line.chars().map(Tile::try_from).collect::<Result<Vec<_>>>()
        }).collect::<Result<Vec<_>>>()?;

        Ok(Map {
            tiles
        })
    }

    fn get(&self, x: usize, y: usize) -> Result<Tile> {
        let row = self.tiles.get(y).context(format!("y out of bounds {}", y))?;
        Ok(row[x % row.len()])
    }

    fn tree_for_slope(&self, dx: usize, dy: usize) -> Result<usize> {
        let mut y = 0;
        let mut x = 0;
        let mut tree_count = 0;

        while y < self.tiles.len() {
            if self.get(x, y)? == Tile::Tree {
                tree_count += 1;
            }

            y += dy;
            x += dx;
        }

        Ok(tree_count)
    }
}

fn part1(input: &str) -> Result<usize> {
    let map = Map::parse(input)?;
    map.tree_for_slope(3, 1)
}

fn part2(input: &str) -> Result<usize> {
    let map = Map::parse(input)?;

    let counts = [
        map.tree_for_slope(1, 1)?,
        map.tree_for_slope(3, 1)?,
        map.tree_for_slope(5, 1)?,
        map.tree_for_slope(7, 1)?,
        map.tree_for_slope(1, 2)?,
    ];

    Ok(counts.iter().product())
}
