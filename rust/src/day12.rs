use std::collections::{HashSet, VecDeque};

use anyhow::{Ok, Result};

type Vertex = (isize, isize);
type Graph = Vec<Vec<u8>>;

fn parse_graph(lines: Vec<String>) -> (Graph, Vertex, Vertex) {
    let mut source = (0, 0);
    let mut target = (0, 0);
    let graph = lines
        .iter()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        source = (row as isize, col as isize);
                        0
                    } else if c == 'E' {
                        target = (row as isize, col as isize);
                        25
                    } else {
                        (c as u8) - 97
                    }
                })
                .collect()
        })
        .collect();

    (graph, source, target)
}

fn find_shortest_path(graph: &Graph, source: Vertex, target: Vertex) -> Option<u32> {
    let rows = graph.len();
    let cols = graph[0].len();
    let mut vertices = VecDeque::new();
    vertices.push_back((source, 0));
    let mut visited = HashSet::new();
    visited.insert(source);
    while let Some(((x, y), dist)) = vertices.pop_front() {
        let neighbours = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for (xn, yn) in neighbours {
            let ux = x as usize;
            let uy = y as usize;
            let uxn = xn as usize;
            let uyn = yn as usize;
            if xn >= 0
                && uxn < rows
                && yn >= 0
                && uyn < cols
                && graph[uxn][uyn] <= graph[ux][uy] + 1
                && visited.insert((xn, yn))
            {
                if (xn, yn) == target {
                    return Some(dist + 1);
                }
                vertices.push_back(((xn, yn), dist + 1));
            }
        }
    }
    None
}

fn part1() -> Result<u32> {
    let lines = aoc::read_file_as_lines::<String>("day12")?;
    let (graph, source, target) = parse_graph(lines);
    Ok(find_shortest_path(&graph, source, target).unwrap())
}

fn part2() -> Result<u32> {
    let lines = aoc::read_file_as_lines::<String>("day12")?;
    let (graph, source, target) = parse_graph(lines);
    let init = find_shortest_path(&graph, source, target).unwrap();

    Ok(graph
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().fold(init, |path, (j, cell)| {
                if *cell == 0 {
                    if let Some(new_path) =
                        find_shortest_path(&graph, (i as isize, j as isize), target)
                    {
                        if new_path < path {
                            new_path
                        } else {
                            path
                        }
                    } else {
                        path
                    }
                } else {
                    path
                }
            })
        })
        .min()
        .unwrap())
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", part1()?);
    println!("Part 2 result: {}", part2()?);
    Ok(())
}
