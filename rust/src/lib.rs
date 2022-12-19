use std::str::FromStr;

use anyhow::Result;

pub fn read_file_as_lines<T>(day: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string("data/".to_string() + day)?
       .lines()
       .filter_map(|l| l.parse::<T>().ok())
       .collect())
}

pub fn read_file_as_string(day: &str) -> Result<String> {
    Ok(std::fs::read_to_string("data/".to_string() + day)?)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhattan_dist(&self, point: &Self) -> isize {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}
