#![cfg(test)]

use std::collections::HashMap;

fn solve1() -> usize {
    include_str!("input.txt").split(',').map(hash).sum()
}

fn solve2() -> usize {
    // Map box number to list of lenses (label and focal length).
    let mut boxes: HashMap<_, Vec<(_, _)>> = HashMap::new();

    for step in include_str!("input.txt").split(',') {
        let mut parts = step.split(&['=', '-'][..]);
        let label = parts.next().unwrap();
        let focal = parts.next().unwrap().parse::<usize>().ok();
        let lenses = boxes.entry(hash(label)).or_insert_with(Vec::new);

        if let Some(focal) = focal {
            // '=': Update or add a lens with that label.

            // Find lens with that label.
            if let Some(lens) = lenses.iter_mut().find(|lens| lens.0 == label) {
                // Found a lens. Update it.
                lens.1 = focal;
            } else {
                // Did not find a lens. Add one.
                lenses.push((label, focal));
            }
        } else {
            // '-': Remove any lenses with that label.
            lenses.retain(|lens| lens.0 != label);
        }
    }

    boxes
        .iter()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, (_, focal))| (box_index + 1) * (lens_index + 1) * focal)
                .sum::<usize>()
        })
        .sum()
}

fn hash(text: &str) -> usize {
    let mut hash = 0;
    for char in text.chars() {
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[test]
fn test_part1() {
    assert_eq!(516_469, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(221_627, solve2());
}
