use std::isize::{MAX, MIN};

use anyhow::{Ok, Result};
use aoc::Point;
use regex::Regex;

struct Map {
    sensors: Vec<(Point, Point)>,
}

impl<I> From<I> for Map
where
    I: Iterator<Item = String>,
{
    fn from(it: I) -> Self {
        let regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .expect("regex creation failed");

        let mut sensors = vec![];
        for s in it {
            let res = regex.captures(&s);
            match res {
                Some(cap) => {
                    let sensor = Point {
                        x: cap[1].parse::<isize>().unwrap(),
                        y: cap[2].parse::<isize>().unwrap(),
                    };
                    let beacon = Point {
                        x: cap[3].parse::<isize>().unwrap(),
                        y: cap[4].parse::<isize>().unwrap(),
                    };
                    sensors.push((sensor, beacon));
                }
                None => panic!("parsing failed"),
            }
        }

        Map { sensors }
    }
}

impl Map {
    fn ranges_for(&self, y: isize) -> Ranges {
        let data = self
            .sensors
            .iter()
            .fold(vec![], |mut acc: Vec<Range>, (s, b)| {
                let dist_y = (s.y - y).abs();
                if dist_y <= s.manhattan_dist(b) {
                    let dist = s.manhattan_dist(b);
                    let offset = dist - dist_y;
                    acc.push(Range::new(s.x - offset, s.x + offset));
                    acc
                } else {
                    acc
                }
            });
        Ranges { data }
    }
}

#[derive(Copy, Clone)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn new(min: isize, max: isize) -> Self {
        Self { min, max }
    }
}

struct Ranges {
    data: Vec<Range>,
}

impl Ranges {
    fn fold_ranges(&mut self) {
        self.data.sort_by(|r1, r2| r1.min.cmp(&r2.min));

        self.data = self.data.iter().fold(vec![], |mut data, r| {
            let last = data.last();
            match last {
                Some(range) => {
                    if range.max < r.min {
                        data.push(*r);
                    } else if range.max <= r.max {
                        let mut new_range = data.pop().unwrap();
                        new_range.max = r.max;
                        data.push(new_range);
                    }
                }
                None => {
                    if r.min < 0 {
                        data.push(Range::new(0, r.max));
                    } else {
                        data.push(*r);
                    }
                }
            }
            data
        })
    }
}

fn part1(map: &Map, y: isize) -> Result<usize> {
    let (min, max) = map
        .ranges_for(y)
        .data
        .iter()
        .fold((MAX, MIN), |(mut min, mut max), range| {
            min = min.min(range.min);
            max = max.max(range.max);
            (min, max)
        });
    Ok((max - min) as usize)
}

fn part2(map: &Map, max: isize) -> Result<usize> {
    let mut point = None;
    for y in 0..=max {
        let mut ranges = map.ranges_for(y);
        ranges.fold_ranges();

        if ranges.data.len() > 1 {
            let x_opt = ranges
                .data
                .windows(2)
                .find_map(|v| match v[0].max + 1 == v[1].min - 1 {
                    true => Some(v[0].max + 1),
                    false => None,
                });

            if let Some(x) = x_opt {
                point = Some((x, y));
                break;
            }
        }
    }

    match point {
        Some((x, y)) => Ok((x * 4_000_000 + y) as usize),
        None => panic!("no answer found"),
    }
}

fn main() -> Result<()> {
    let map = Map::from(aoc::read_file_as_lines::<String>("day15")?.into_iter());
    println!("Part 1 result: {}", part1(&map, 2_000_000)?);
    println!("Part 2 result: {}", part2(&map, 4_000_000)?);

    Ok(())
}
