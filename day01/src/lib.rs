#![cfg(test)]

use std::collections::HashMap;

fn solve() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap();
            let last = line.chars().filter(char::is_ascii_digit).last().unwrap();

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

fn solve2() -> u32 {
    let mut mapping = HashMap::new();
    mapping.insert("0", '0');
    mapping.insert("1", '1');
    mapping.insert("2", '2');
    mapping.insert("3", '3');
    mapping.insert("4", '4');
    mapping.insert("5", '5');
    mapping.insert("6", '6');
    mapping.insert("7", '7');
    mapping.insert("8", '8');
    mapping.insert("9", '9');
    mapping.insert("one", '1');
    mapping.insert("two", '2');
    mapping.insert("three", '3');
    mapping.insert("four", '4');
    mapping.insert("five", '5');
    mapping.insert("six", '6');
    mapping.insert("seven", '7');
    mapping.insert("eight", '8');
    mapping.insert("nine", '9');

    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut first_index = None;
            let mut first = None;
            for (k, v) in &mapping {
                if let Some(index) = line.find(k) {
                    if first_index.is_none() || index < first_index.unwrap() {
                        first_index = Some(index);
                        first = Some(v);
                    }
                }
            }

            let mut last_index = None;
            let mut last = None;
            for (k, v) in &mapping {
                if let Some(index) = line.rfind(k) {
                    if last_index.is_none() || index > last_index.unwrap() {
                        last_index = Some(index);
                        last = Some(v);
                    }
                }
            }

            let first = first.unwrap();
            let last = last.unwrap();
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(55_447, solve());
}

#[test]
fn test_part2() {
    assert_eq!(54_706, solve2());
}
