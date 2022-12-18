
use std::collections::{HashSet, HashMap, LinkedList};

fn parse(lines: core::str::Lines) -> HashSet<(i32, i32, i32)> {
    let mut result: HashSet<(i32, i32, i32)> = HashSet::<(i32, i32, i32)>::new();

    for line in lines {
        let nums: Vec<i32> = line.split(',').map(|val| val.parse::<i32>().unwrap()).collect();
        result.insert((nums[0], nums[1], nums[2]));
    }

    result
}

fn surface_area(cubes: &HashSet<(i32, i32, i32)>) -> usize {
    let offsets = [
        ( 1, 0, 0 ), ( -1, 0, 0),
        ( 0, 1, 0 ), ( 0, -1, 0),
        ( 0, 0, 1 ), ( 0, 0, -1),
    ];

    cubes.iter().map(|cube|
        6 - offsets.iter().filter(|offset| cubes.contains(&(cube.0 + offset.0, cube.1+ offset.1, cube.2 + offset.2))).count()
    ).sum()
}

fn cube_extents(cubes: &HashSet<(i32, i32, i32)>) -> ((i32, i32, i32), (i32, i32, i32)) {
    (
        (
            cubes.iter().map(|c| c.0).min().unwrap() - 1,
            cubes.iter().map(|c| c.1).min().unwrap() - 1,
            cubes.iter().map(|c| c.2).min().unwrap() - 1,
        ),
        (
            cubes.iter().map(|c| c.0).max().unwrap() + 1,
            cubes.iter().map(|c| c.1).max().unwrap() + 1,
            cubes.iter().map(|c| c.2).max().unwrap() + 1,
        )
    )
}

fn exterior_cubes(cubes: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let offsets = [
        ( 1, 0, 0 ), ( -1, 0, 0),
        ( 0, 1, 0 ), ( 0, -1, 0),
        ( 0, 0, 1 ), ( 0, 0, -1),
    ];

    let mut result: HashSet<(i32, i32, i32)> = HashSet::<(i32, i32, i32)>::new();

    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = cube_extents(cubes);

    let mut worklist: LinkedList<(i32, i32, i32)> = LinkedList::<(i32, i32, i32)>::new();
    result.insert((min_x, min_y, min_z));
    worklist.push_back((min_x, min_y, min_z));

    while let Some(pos) = worklist.pop_front() {
        for offset in offsets {
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2);
            if !cubes.contains(&new_pos) && !result.contains(&new_pos) {
                if new_pos.0 >= min_x && new_pos.0 <= max_x && new_pos.1 >= min_y && new_pos.1 <= max_y && new_pos.2 >= min_z && new_pos.2 <= max_z {
                    worklist.push_back(new_pos);
                    result.insert(new_pos);
                }
            }
        }
    }

    result
}

fn interior_cubes(cubes: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut result: HashSet<(i32, i32, i32)> = HashSet::<(i32, i32, i32)>::new();
    let exterior_cubes = exterior_cubes(cubes);

    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = cube_extents(cubes);

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            for z in min_z..max_z + 1 {
                if !cubes.contains(&(x, y, z)) && !exterior_cubes.contains(&(x, y, z)) {
                    println!("{},{},{}", x, y, z);
                    result.insert((x, y, z));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part1() {
        const INPUT: &str = include_str!("../inputs/day18.txt");
        let cubes = parse(INPUT.lines());

        println!("{}", surface_area(&cubes));
    }

    #[test]
    fn day18_part2() {
        const INPUT: &str = include_str!("../inputs/day18.txt");
        let mut cubes = parse(INPUT.lines());
        let interior_cubes = interior_cubes(&cubes);

        for cube in interior_cubes {
            cubes.insert(cube);
        }

        println!("{}", surface_area(&cubes));
    }
}
