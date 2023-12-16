#![cfg(test)]

use std::cmp::max;
use std::collections::{HashMap, HashSet};

type Tile = char;
type Position = (isize, isize);
type Direction = char;
type Beam = (Position, Direction);

fn solve1() -> usize {
    let tiles = parse();

    simulate(&tiles, ((0, 0), '>'))
}

fn solve2() -> usize {
    let tiles = parse();
    let max_x = tiles.keys().map(|position| position.0).max().unwrap();
    let max_y = tiles.keys().map(|position| position.1).max().unwrap();

    let mut result = 0;
    for x in 0..=max_x {
        result = max(result, simulate(&tiles, ((x, 0), 'v')));
        result = max(result, simulate(&tiles, ((x, max_y), '^')));
    }
    for y in 0..=max_y {
        result = max(result, simulate(&tiles, ((0, y), '>')));
        result = max(result, simulate(&tiles, ((max_x, y), '<')));
    }

    result
}

#[allow(clippy::cast_possible_wrap)]
fn parse() -> HashMap<Position, Tile> {
    let mut tiles = HashMap::new();
    for (y, line) in include_str!("input.txt").lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            tiles.insert((x as isize, y as isize), tile);
        }
    }
    tiles
}

fn simulate(tiles: &HashMap<Position, Tile>, start: Beam) -> usize {
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut to_visit: Vec<Beam> = vec![start];

    while let Some(((x, y), direction)) = to_visit.pop() {
        if let Some(tile) = tiles.get(&(x, y)) {
            if !visited.insert(((x, y), direction)) {
                continue;
            }

            #[rustfmt::skip]
            #[allow(clippy::match_same_arms)]
            let next_directions = match (direction, tile) {
                ('>', '-' | '.') => ">",
                ('>', '\\')      => "v",
                ('>', '/')       => "^",
                ('>', '|')       => "^v",

                ('<', '-' | '.') => "<",
                ('<', '\\')      => "^",
                ('<', '/')       => "v",
                ('<', '|')       => "^v",

                ('v', '|' | '.') => "v",
                ('v', '\\')      => ">",
                ('v', '/')       => "<",
                ('v', '-')       => "<>",

                ('^', '|' | '.') => "^",
                ('^', '\\')      => "<",
                ('^', '/')       => ">",
                ('^', '-')       => "<>",

                _ => unreachable!(),
            };

            for next_direction in next_directions.chars() {
                let (dx, dy) = match next_direction {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    'v' => (0, 1),
                    '^' => (0, -1),
                    _ => unreachable!(),
                };

                to_visit.push(((x + dx, y + dy), next_direction));
            }
        }
    }

    // Get number of distinct positions visited.
    visited
        .iter()
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
        .len()
}

#[test]
fn test_part1() {
    assert_eq!(7_951, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(8_148, solve2());
}
