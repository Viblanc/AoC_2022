use anyhow::Result;
use aoc::read_file_as_lines;

fn count_calories() -> Result<Vec<u32>> {
    let input = read_file_as_lines::<String>("day1")?;
    let elves_calories: Vec<Vec<u32>> = input.split(|s| s.len() == 0)
        .map(|slice| slice.into_iter().map(|s| s.parse::<u32>().unwrap()).collect())
        .collect();

    let total_calories: Vec<u32> = elves_calories.into_iter()
        .map(|v| v.into_iter().sum())
        .collect();

    Ok(total_calories)
}

fn part1() -> Result<u32> {
    Ok(
        count_calories()?
            .into_iter()
            .reduce(|max, cur| if max < cur { cur } else { max })
            .unwrap()
    )
}

fn part2() -> Result<u32> {
    let mut calories_vec = count_calories()?.to_vec();
    calories_vec.sort_by(|a, b| b.cmp(a));
    Ok(
        calories_vec[0..3].into_iter().sum()
    )
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);

    Ok(())
}
