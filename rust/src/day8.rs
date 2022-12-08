use anyhow::Result;

fn find_visible_trees(trees: Vec<Vec<u32>>) -> Result<u32> {
    let rows = trees.len();
    let cols = trees[0].len();
    let mut n = rows * 2 + cols * 2 - 4;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if (0..row).all(|e| trees[e][col] < trees[row][col])
                || ((row + 1)..rows).all(|e| trees[e][col] < trees[row][col])
                || (0..col).all(|e| trees[row][e] < trees[row][col])
                || ((col + 1)..cols).all(|e| trees[row][e] < trees[row][col])
            {
                n += 1;
            }
        }
    }

    Ok(n as u32)
}

fn get_highest_score(trees: Vec<Vec<u32>>) -> Result<u32> {
    let rows = trees.len();
    let cols = trees[0].len();

    let mut high_score = 0;

    for (row, tree_line) in trees.iter().enumerate() {
        for (col, tree_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            for i in (0..row).rev() {
                if trees[i][col] < *tree_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }

            for i in (row + 1)..rows {
                if trees[i][col] < *tree_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            for i in (0..col).rev() {
                if trees[row][i] < *tree_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }

            for i in (col + 1)..cols {
                if trees[row][i] < *tree_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }

            let scenic_score = scores.iter().product();

            if high_score < scenic_score {
                high_score = scenic_score;
            }
        }
    }

    Ok(high_score)
}

fn get_map(input: Vec<String>) -> Result<Vec<Vec<u32>>> {
    Ok(input
       .iter()
       .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
       .collect())
}

fn part1(file: &str) -> Result<u32> {
    let input = aoc::read_file_as_lines::<String>(file)?;
    find_visible_trees(get_map(input)?)
}

fn part2(file: &str) -> Result<u32> {
    let input = aoc::read_file_as_lines::<String>(file)?;
    get_highest_score(get_map(input)?)
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", part1("day8")?);
    println!("Part 2 result: {}", part2("day8")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use core::fmt;

    use anyhow::bail;

    use super::*;

    #[derive(Debug)]
    struct MyError {
        actual: u32,
        expected: u32,
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "expected {} but got {}", self.expected, self.actual)
        }
    }

    #[test]
    fn test_part1() -> Result<()>{
        let result = part1("day8_test")?;
        if result == 21 {
            Ok(())
        } else {
            bail!(MyError { actual: result, expected: 21 })
        }
    }

    #[test]
    fn test_part2() -> Result<()>{
        let result = part2("day8_test")?;
        if result == 8 {
            Ok(())
        } else {
            bail!(MyError { actual: result, expected: 8 })
        }
    }
}
