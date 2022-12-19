use std::collections::{HashMap, hash_map::RandomState, HashSet};

use anyhow::Result;

fn items_priorities() -> HashMap<char, u32, RandomState> {
    let mut item_types = HashMap::new();
    let mut i = 1;
    for c in 'a'..='z' {
        item_types.insert(c, i);
        item_types.insert(c.to_ascii_uppercase(), i + 26);
        i += 1;
    }
    return item_types;
}

fn part1() -> Result<u32> {
    Ok(
        aoc::read_file_as_lines::<String>("day3")?
            .into_iter()
            .map(|s| {
                let (fst, snd) = s.split_at(s.len() / 2);
                let first_compartment: HashSet<char> = fst.chars().into_iter().collect();
                let mut sum = 0;
                for c in snd.chars() {
                    if let Some(_) = first_compartment.get(&c) {
                        sum += items_priorities().get(&c).unwrap();
                        break;
                    }
                }
                sum
            })
            .sum()
    )
}

fn part2() -> Result<u32> {
    Ok(
        aoc::read_file_as_lines::<String>("day3")?
            .chunks(3)
            .into_iter()
            .map(|r| {
                let rucksacks: Vec<HashSet<char>> = r.into_iter()
                    .map(|s| s.chars().into_iter().collect())
                    .collect();

                let mut priority = 0;

                for c in &rucksacks[0] {
                    if let Some(_) = rucksacks[1].get(c) {
                        if let Some(_) = rucksacks[2].get(c) {
                            priority += items_priorities().get(c).unwrap();
                            break;
                        }
                    }
                }
                priority
            })
            .sum()
    )
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", part1()?);
    println!("Part 2 result: {}", part2()?);

    Ok(())
}
