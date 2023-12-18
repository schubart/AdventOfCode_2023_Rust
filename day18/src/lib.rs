#![cfg(test)]

fn solve(points: &[(isize, isize)]) -> isize {
    // Pair each point with its successor.
    let pairs = points.iter().zip(points.iter().cycle().skip(1));

    // Shoelace formula for area of a simple polygon:
    // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula
    let area = pairs
        .clone()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<isize>()
        / 2;

    // The area calculated above is only the area described by the centers of each tile and ignores
    // the sizes (1*1) of the tiles themselves. March along the perimeter and add half a tile size
    // for each step. For example, for 2*2 tiles, `area` is 1, so add (4 / 2) + 1 = 3 for the
    // perimeter. This represents the area of the missing corners: 4 * 3/4 = 3. The total area is 4.
    //
    // Visually (1 character is 1/4 tile):
    //
    // PPPP
    // PAAP
    // PAAP
    // PPPP
    let perimeter_area = pairs
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs())
        .sum::<isize>()
        / 2
        + 1;

    area + perimeter_area
}

fn parse1() -> Vec<(isize, isize)> {
    let mut points = Vec::new();

    let (mut x, mut y) = (0, 0);
    for line in include_str!("input.txt").lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<isize>().unwrap();

        (x, y) = match direction {
            "U" => (x, y - distance),
            "D" => (x, y + distance),
            "L" => (x - distance, y),
            "R" => (x + distance, y),
            _ => unreachable!(),
        };

        points.push((x, y));
    }

    points
}

fn parse2() -> Vec<(isize, isize)> {
    let mut points = Vec::new();

    let (mut x, mut y) = (0, 0);
    for line in include_str!("input.txt").lines() {
        let code = line.split(&['#', ')'][..]).nth(1).unwrap();
        let (distance, direction) = code.split_at(code.len() - 1);
        let distance = isize::from_str_radix(distance, 16).unwrap();

        (x, y) = match direction {
            "3" => (x, y - distance),
            "1" => (x, y + distance),
            "2" => (x - distance, y),
            "0" => (x + distance, y),
            _ => unreachable!(),
        };

        points.push((x, y));
    }

    points
}

#[test]
fn test_part1() {
    assert_eq!(50_603, solve(&parse1()));
}

#[test]
fn test_part2() {
    assert_eq!(96_556_251_590_677, solve(&parse2()));
}
