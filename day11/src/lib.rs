#![cfg(test)]

use std::collections::HashSet;

fn solve(factor: usize) -> usize {
    let mut galaxies = HashSet::new();
    for (y, line) in include_str!("input.txt").lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    // Find the columns without galaxies.
    let width = galaxies.iter().map(|(x, _)| x).max().unwrap() + 1;
    let cols: Vec<_> = (0..width)
        .filter(|&col| !galaxies.iter().any(|&(x, _)| x == col))
        .collect();

    // Find the rows without galaxies.
    let height = galaxies.iter().map(|(_, y)| y).max().unwrap() + 1;
    let rows: Vec<_> = (0..height)
        .filter(|&row| !galaxies.iter().any(|&(_, y)| y == row))
        .collect();

    let positions: HashSet<_> = galaxies
        .iter()
        .map(|(x, y)| {
            // Find number of empty columns and rows left and above this galaxy.
            let cols = cols.iter().filter(|&col| col < x).count();
            let rows = rows.iter().filter(|&row| row < y).count();

            // Move galaxy by that much.
            (x + cols * (factor - 1), y + rows * (factor - 1))
        })
        .collect();

    let mut result = 0;
    for &(x1, y1) in &positions {
        for &(x2, y2) in &positions {
            result += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }

    // Do not double-count.
    result / 2
}

#[test]
fn test_part1() {
    assert_eq!(9_609_130, solve(2));
}

#[test]
fn test_part2() {
    assert_eq!(702_152_204_842, solve(1_000_000));
}
