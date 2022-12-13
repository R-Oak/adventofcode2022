
use std::collections::LinkedList;

fn parse(lines: core::str::Lines) -> Vec<Vec<u8>> {
    lines
        .map(|line| line.bytes()
            .map(|b| match b {
                b'S' => 0,
                b'E' => 27,
                _ => b - b'a' + 1
            }).collect()
        ).collect()
}

fn find(height: u8, heights: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut x = 0;
    while x < heights.len() {
        let mut y = 0;
        while y < heights[0].len() {
            if heights[x][y] == height {
                return (x, y);
            }
            y += 1;
        }
        x += 1;
    }

    panic!("Cannot find height {}", height);
}

fn fill_distance(x: usize, y: usize, height: u8, dist: i32, heights: &Vec<Vec<u8>>, dists: &mut Vec<Vec<Option<i32>>>) -> bool {
    let mut result: bool = true;

    if height == 0 {
        return false;
    }

    if heights[x][y] < height - 1 {
        return false;
    }

    dists[x][y] = match dists[x][y] {
        Some(d) => {
            if dist < d {
                Some(dist)
            } else {
                result = false;
                Some(d)
            }
        }
        None => Some(dist)
    };

    result
}

fn flood_fill_tile(x: usize, y: usize, heights: &Vec<Vec<u8>>, dists: &mut Vec<Vec<Option<i32>>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let d = dists[x][y].unwrap();
    let h: u8 = heights[x][y];

    if x > 0 {
        if fill_distance(x - 1, y, h, d + 1, heights, dists) {
            result.push((x - 1, y));
        }
    }

    if x < heights.len() - 1 {
        if fill_distance(x + 1, y, h, d + 1, heights, dists) {
            result.push((x + 1, y));
        }
    }

    if y > 0 {
        if fill_distance(x, y - 1, h, d + 1, heights, dists) {
            result.push((x, y - 1));
        }
    }

    if y < heights[0].len() - 1 {
        if fill_distance(x, y + 1, h, d + 1, heights, dists) {
            result.push((x, y + 1));
        }
    }

    result
}

fn flood_fill(start_x: usize, start_y: usize, heights: &Vec<Vec<u8>>, dists: &mut Vec<Vec<Option<i32>>>) {
    let mut worklist: LinkedList<(usize, usize)> = LinkedList::new();
    worklist.push_back((start_x, start_y));

    while let Some((x, y)) = worklist.pop_front() {
        let next_cells = flood_fill_tile(x, y, heights, dists);
        for cell in next_cells {
            // println!("({},{})->({},{})", x, y, cell.0, cell.1);
            worklist.push_back(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1() {
        const INPUT: &str = include_str!("../inputs/day12.txt");
        let heights: Vec<Vec<u8>> = parse(INPUT.lines());
        let mut dists: Vec<Vec<Option<i32>>> = heights.iter()
            .map(|row| row.iter()
                .map(|_elem| None)
                .collect())
            .collect();

            let (sx, sy) = find(0, &heights);
            let (tx, ty) = find(27, &heights);
            dists[tx][ty] = Some(0);

        flood_fill(tx, ty, &heights, &mut dists);

        for row in &dists {
            for cell in row {
                match cell {
                    Some(d) => print!("{:>2} ", d),
                    None => print!("-1 "),
                }
            }
            print!("\n");
        }

        println!("{}", dists[sx][sy].unwrap());
    }

    #[test]
    fn day12_part2() {
        const INPUT: &str = include_str!("../inputs/day12.txt");
        let heights: Vec<Vec<u8>> = parse(INPUT.lines());
        let mut dists: Vec<Vec<Option<i32>>> = heights.iter()
            .map(|row| row.iter()
                .map(|_elem| None)
                .collect())
            .collect();

            let (sx, sy) = find(0, &heights);
            let (tx, ty) = find(27, &heights);
            dists[tx][ty] = Some(0);

        flood_fill(tx, ty, &heights, &mut dists);

        let mut result:Vec<i32> = vec![];

        for (x, row) in heights.iter().enumerate() {
            for (y, h) in row.iter().enumerate() {
                if h == &1 {
                    match dists[x][y] {
                        Some(d) => result.push(d),
                        None => (),
                    }
                }
            }
        }

        let minimum = result.iter().min().unwrap();
        println!("{}", minimum);
    }
}
