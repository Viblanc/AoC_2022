use anyhow::Result;

fn get_first_start_of_packet_marker(win_size: usize) -> Result<u32> {
    Ok(aoc::read_file::<String>("day6")?[0]
        .chars()
        .collect::<Vec<char>>()
        .windows(win_size)
        .enumerate()
        .find_map(|(idx, chars)| {
            let mut dups = chars.to_vec();
            dups.sort();
            dups.dedup();
            match dups.len() == chars.len() {
                true => Some((win_size + idx) as u32),
                false => None,
            }
        })
        .unwrap())
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", get_first_start_of_packet_marker(4)?);
    println!("Part 2 result: {}", get_first_start_of_packet_marker(14)?);

    Ok(())
}
