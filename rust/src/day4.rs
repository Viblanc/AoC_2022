use std::ops::Range;

use anyhow::Result;

struct MyRange {
    start: u8,
    end: u8,
}

impl MyRange {
    fn from_range(range: Range<u8>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    fn contains(&self, range: &Self) -> bool {
        self.start <= range.start && self.end >= range.end
    }

    fn overlaps(&self, range: &Self) -> bool {
        (self.start >= range.start && self.start <= range.end)
            || (self.end >= range.start && self.end <= range.end)
    }
}

fn str_to_pair<T>(str: &str, sep: &str, func: fn(&str) -> T) -> (T, T) {
    let split: Vec<&str> = str.split(sep).collect();
    let first = split[0];
    let second = split[1];
    (func(first), func(second))
}

fn get_range_pair(str: &str) -> (MyRange, MyRange) {
    let to_string = |s: &str| -> String { s.to_string() };
    let to_u8 = |s: &str| -> u8 { s.parse::<u8>().unwrap() };
    let (range1, range2) = str_to_pair::<String>(str, ",", to_string);
    let (start1, end1) = str_to_pair::<u8>(&range1, "-", to_u8);
    let (start2, end2) = str_to_pair::<u8>(&range2, "-", to_u8);
    (
        MyRange::from_range(start1..end1),
        MyRange::from_range(start2..end2),
    )
}

fn filter_ranges(predicate: fn (s: &MyRange, r: &MyRange) -> bool) -> Result<u32> {
    Ok(aoc::read_file::<String>("day4")?
        .iter()
        .map(|line| {
            let (range1, range2) = get_range_pair(line);
            if predicate(&range1, &range2) || predicate(&range2, &range1) {
                1
            } else {
                0
            }
        })
        .sum())
}

fn part1() -> Result<u32> {
    filter_ranges(MyRange::contains)
}

fn part2() -> Result<u32> {
    filter_ranges(MyRange::overlaps)
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", part1()?);
    println!("Part 2 result: {}", part2()?);

    Ok(())
}
