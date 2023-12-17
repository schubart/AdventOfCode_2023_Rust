#![cfg(test)]

use std::collections::{BinaryHeap, HashMap};

type Position = (isize, isize);
type Node = (Position, bool);
type Cost = isize;

fn solve(min_steps: isize, max_steps: isize) -> Cost {
    let mut tiles = HashMap::new();
    for (y, line) in include_str!("input.txt").lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            #[allow(clippy::cast_possible_wrap)]
            tiles.insert(
                (x as isize, y as isize),
                tile.to_digit(10).unwrap() as isize,
            );
        }
    }

    let start = (0, 0);
    let finish = (
        tiles.keys().map(|pos| pos.0).max().unwrap(),
        tiles.keys().map(|pos| pos.1).max().unwrap(),
    );

    // Dijkstra's Algorithm
    let mut costs: HashMap<Node, Cost> = HashMap::new();
    costs.insert((start, true), 0);
    costs.insert((start, false), 0);

    let mut queue: BinaryHeap<(Cost, Node)> = BinaryHeap::new();
    queue.push((0, (start, true)));
    queue.push((0, (start, false)));

    loop {
        // This can only fail if there is no path to the finish.
        let (cost, (position @ (x, y), direction)) = queue.pop().unwrap();
        // Queue uses negative cost in order to prefer lower cost. Get actual cost.
        let cost = -cost;

        if position == finish {
            break cost;
        }

        let deltas = if direction {
            [(-1, 0), (1, 0)]
        } else {
            [(0, -1), (0, 1)]
        };

        for (dx, dy) in deltas {
            let mut new_cost = cost;

            for steps in 1..=max_steps {
                let new_position = (x + dx * steps, y + dy * steps);

                if let Some(tile_cost) = tiles.get(&new_position) {
                    new_cost += tile_cost;

                    if steps >= min_steps {
                        let new_node = (new_position, !direction);

                        if costs.get(&new_node).map_or(true, |&c| new_cost < c) {
                            costs.insert(new_node, new_cost);
                            queue.push((-new_cost, new_node));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(907, solve(1, 3));
}

#[test]
fn test_part2() {
    assert_eq!(1_057, solve(4, 10));
}
