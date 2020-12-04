use std::{collections::HashMap, convert::TryFrom, str::FromStr};

use anyhow::{anyhow, Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);

    Ok(())
}

struct Passport {
    fields: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split_whitespace()
            .map(|field| {
                let mut parts = field.split(":");
                Ok((
                    parts.next().context("invalid field")?.into(),
                    parts.next().context("invalid field")?.into(),
                ))
            })
            .collect::<Result<HashMap<_, _>, anyhow::Error>>()?;

        Ok(Passport { fields })
    }
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
    }

    fn validate(&self) -> Result<()> {
        if !self.has_required_fields() {
            return Err(anyhow!("not all fields present"));
        }

        let byr = self.fields["byr"].parse::<isize>()?;
        if byr < 1920 || byr > 2002 {
            return Err(anyhow!("invalid byr"));
        }

        let iyr = self.fields["iyr"].parse::<isize>()?;
        if iyr < 2010 || iyr > 2020 {
            return Err(anyhow!("invalid iyr"));
        }

        let eyr = self.fields["eyr"].parse::<isize>()?;
        if eyr < 2020 || iyr > 2030 {
            return Err(anyhow!("invalid eyr"));
        }

        let hgt = &self.fields["hgt"];
        let cm = hgt.ends_with("cm");
        let inch = hgt.ends_with("in");
        if !(cm || inch) {
            return Err(anyhow!("invalid hgt"));
        }
        dbg!(&hgt);
        let hgt = hgt[0..hgt.len() - 2].parse::<isize>()?;
        dbg!(&hgt);
        if (cm && (hgt < 150 || hgt > 193)) || (inch && (hgt < 59 || hgt > 76)) {
            return Err(anyhow!("invalid hgt"));
        }

        let hcl = &self.fields["hcl"];
        if hcl.chars().count() != 7 {
            return Err(anyhow!("invalid hcl"));
        }
        if !hcl.starts_with("#") {
            return Err(anyhow!("invalid hcl"));
        }
        for c in hcl.chars().skip(1) {
            if !(c >= '0' && c <= '9') && !(c >= 'a' && c <= 'f') {
                return Err(anyhow!("invalid hcl"));
            }
        }

        let ecl = &self.fields["ecl"];
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()) {
            return Err(anyhow!("invalid ecl"));
        }

        let pid = &self.fields["pid"];
        if pid.chars().count() != 9 {
            return Err(anyhow!("invalid pid"));
        }
        for c in pid.chars() {
            if c < '0' || c > '9' {
                return Err(anyhow!("invalid pid"));
            }
        }

        Ok(())
    }
}

fn parse_passports(input: &str) -> Result<Vec<Passport>> {
    // Kind of a hacky way to split
    input.split("\n\n").map(str::parse).collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse_passports(input)?
        .iter()
        .filter(|p| p.has_required_fields())
        .count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(parse_passports(input)?
        .iter()
        .filter(|p| p.validate().is_ok())
        .count())
}
