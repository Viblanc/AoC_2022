use anyhow::{Ok, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

enum Packet {
    List(Vec<Packet>),
    Val(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Val(l0), Self::Val(r0)) => l0 == r0,
            (Self::List(l0), Self::Val(r0)) => l0 == &vec![Self::Val(*r0)],
            (Self::Val(l0), Self::List(r0)) => &vec![Self::Val(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Val(l0), Self::Val(r0)) => l0.partial_cmp(r0),
            (Self::Val(l0), Self::List(r0)) => vec![Self::Val(*l0)].partial_cmp(r0),
            (Self::List(l0), Self::Val(r0)) => l0.partial_cmp(&vec![Self::Val(*r0)]),
            (Self::List(l0), Self::List(r0)) => l0.partial_cmp(r0),
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Val(l0), Self::Val(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::Val(r0)) => l0.cmp(&vec![Self::Val(*r0)]),
            (Self::Val(l0), Self::List(r0)) => vec![Self::Val(*l0)].cmp(r0),
            (Self::List(l0), Self::List(r0)) => l0.cmp(r0),
        }
    }
}

type Pair = (Packet, Packet);

fn packet_parser(line: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet_parser), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|val| Packet::Val(val)),
    ))(line)
}

fn pair_parser(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet_parser, newline, packet_parser),
    )(input)
}

fn part1(input: &str) -> Result<usize> {
    let (_, pairs) = pair_parser(input).unwrap();
    Ok(pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
        .fold(0, |acc, idx| acc + idx))
}

fn part2(input: &str) -> Result<usize> {
    let (_, pairs) = pair_parser(input).unwrap();
    let mut packets: Vec<_> = pairs
        .iter()
        .flat_map(|(p1, p2)| vec![p1, p2].into_iter())
        .collect();
    let dividers = [2, 6].map(|i| Packet::List(vec![Packet::List(vec![Packet::Val(i)])]));
    packets.push(&dividers[0]);
    packets.push(&dividers[1]);
    packets.sort();
    Ok(packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p == &&dividers[0] || p == &&dividers[1] {
            true => Some(i + 1),
            false => None,
        })
        .fold(1, |acc, idx| acc * idx))
}

fn main() -> Result<()> {
    let input = aoc::read_file_as_string("day13")?;
    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(part1(INPUT)?, 13);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(part2(INPUT)?, 140);
        Ok(())
    }
}
