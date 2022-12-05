// this is less dumb once split_once() is stable :(

use std::collections::HashMap;

use anyhow::{anyhow, bail};

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

struct Passport(HashMap<&'static str, &'static str>);

impl Passport {
    pub fn is_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|field| self.0.contains_key(field))
    }

    pub fn is_deeply_valid(&self) -> anyhow::Result<()> {
        let v =
            (&self.0.get("byr").ok_or_else(|| anyhow!("missing byr"))?[1..]).parse::<usize>()?;
        if !(1920..=2002).contains(&v) {
            bail!("invalid byr");
        }

        let v =
            (&self.0.get("iyr").ok_or_else(|| anyhow!("missing iyr"))?[1..]).parse::<usize>()?;
        if !(2010..=2020).contains(&v) {
            bail!("invalid iyr");
        }

        let v =
            (&self.0.get("eyr").ok_or_else(|| anyhow!("missing eyr"))?[1..]).parse::<usize>()?;
        if !(2020..=2030).contains(&v) {
            bail!("invalid eyr");
        }

        let v = &self.0.get("hgt").ok_or_else(|| anyhow!("missing hgt"))?[1..];
        let d = &v[v.len() - 2..];
        let v = (&v[..v.len() - 2]).parse::<usize>()?;
        if d == "cm" {
            if !(150..=193).contains(&v) {
                bail!("invalid hgt");
            }
        } else if d == "in" {
            if !(59..=76).contains(&v) {
                bail!("invalid hgt");
            }
        } else {
            bail!("invalid hgt");
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let v = &self.0.get("hcl").ok_or_else(|| anyhow!("missing hcl"))?[1..];
        if v.chars().next().ok_or_else(|| anyhow!("invalid hcl"))? != '#' {
            bail!("invalid hcl");
        }
        i64::from_str_radix(&v[1..], 16)?;

        let v = &self.0.get("ecl").ok_or_else(|| anyhow!("missing ecl"))?[1..];
        if !VALID_EYE_COLORS.contains(&v) {
            bail!("invalid ecl");
        }

        let v = &self.0.get("pid").ok_or_else(|| anyhow!("missing pid"))?[1..];
        if v.len() != 9 {
            bail!("invalid pid");
        }
        v.parse::<usize>()?;

        Ok(())
    }
}

fn part1(passports: impl AsRef<[Passport]>) {
    let passports = passports.as_ref();

    let valid = passports.iter().filter(|x| x.is_valid()).count();

    println!("{} of {} passorts valid", valid, passports.len());
}

fn part2(passports: impl AsRef<[Passport]>) {
    let passports = passports.as_ref();

    let valid = passports
        .iter()
        .filter(|x| x.is_deeply_valid().is_ok())
        .count();

    println!("{} of {} passorts deeply valid", valid, passports.len());
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.split("\n\n").filter(|x| !x.is_empty()).collect();

    let passports: Vec<Passport> = lines
        .iter()
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| field.split_at(field.find(':').unwrap()))
                .collect::<HashMap<&str, &str>>()
        })
        .map(Passport)
        .collect();

    part1(&passports);
    part2(&passports);
}
