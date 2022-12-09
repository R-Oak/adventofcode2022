
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn match_direction(dir: &str) -> Direction {
    match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction {}", dir)
    }
}

fn adjacent(hx: i32, hy: i32, tx: i32, ty: i32) -> bool {
    tx >= hx - 1 && tx <= hx + 1 && ty >= hy - 1 && ty <= hy + 1
}

fn move_tail(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    if adjacent(hx, hy, tx, ty) {
        return (tx, ty);
    }

    let xadd: i32 = if hx > tx {
        1
    } else if hx < tx {
        -1
    } else {
        0
    };
    let yadd: i32 = if hy > ty {
        1
    } else if hy < ty {
        -1
    } else {
        0
    };

    (tx + xadd, ty + yadd)
}

fn do_it(lines: core::str::Lines) -> usize {
    let mut set = HashSet::<(i32, i32)>::new();
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    set.insert((tx, ty));

    for line in lines {
        if let Some((dir, count)) = line.split_once(" ") {
            let count: i64 = count.parse::<i64>().unwrap();

            for _ in 0..count {
                match match_direction(dir) {
                    Direction::Up => hy -= 1,
                    Direction::Down => hy += 1,
                    Direction::Left => hx -= 1,
                    Direction::Right => hx += 1,
                }

                (tx, ty) = move_tail(hx, hy, tx, ty);
                set.insert((tx, ty));
            }
        } else {
            panic!("Bad line {}", line);
        }
    }

    set.len()
}

fn do_it2(lines: core::str::Lines) -> usize {
    let mut set = HashSet::<(i32, i32)>::new();
    let mut vec: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    for _ in 0..10 {
        vec.push((0, 0));
    }

    set.insert(vec[0]);

    for line in lines {
        if let Some((dir, count)) = line.split_once(" ") {
            let count: i64 = count.parse::<i64>().unwrap();

            for _ in 0..count {
                let mut new_vec: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

                for (mut x, mut y) in vec {
                    if new_vec.len() == 0 {
                        match match_direction(dir) {
                            Direction::Up => y -= 1,
                            Direction::Down => y += 1,
                            Direction::Left => x -= 1,
                            Direction::Right => x += 1,
                        }
                        new_vec.push((x, y));
                    } else {
                        let (x1, y1) = *new_vec.last().unwrap();
                        new_vec.push(move_tail(x1, y1, x, y));
                    }
                }

                set.insert(*new_vec.last().unwrap());
                vec = new_vec;
            }
        } else {
            panic!("Bad line {}", line);
        }
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1() {
        const INPUT: &str = include_str!("../inputs/day09.txt");

        println!("{}", do_it(INPUT.lines()));
    }

    #[test]
    fn day09_part2() {
        const INPUT: &str = include_str!("../inputs/day09.txt");

        println!("{}", do_it2(INPUT.lines()));
    }
}
