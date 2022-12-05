use std::str::FromStr;

use anyhow::Result;

pub fn read_file<T>(day: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string("data/".to_string() + day)?
       .lines()
       .filter_map(|l| l.parse::<T>().ok())
       .collect())
}
