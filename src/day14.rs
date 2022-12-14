
use std::collections::HashMap;

enum CellType {
    Rock,
    Sand,
    Air,
}

struct Cave {
    cave: HashMap<(i32, i32), CellType>,
}

impl Cave {
    fn new(lines: core::str::Lines) -> Cave {
        let mut cave = HashMap::<(i32, i32), CellType>::new();

        for line in lines {
            let verts: Vec<Vec<i32>> = line.split(" -> ")
                .map(|v: &str|
                    v.split(",").map(|x: &str| x.parse::<i32>().unwrap()
                ).collect()
            ).collect();

            for v in verts.windows(2) {
                let minx: i32 = v[0][0].min(v[1][0]);
                let maxx: i32 = v[0][0].max(v[1][0]);
                let miny: i32 = v[0][1].min(v[1][1]);
                let maxy: i32 = v[0][1].max(v[1][1]);

                for x in minx..maxx+1 {
                    for y in miny..maxy+1 {
                        cave.insert((x, y), CellType::Rock);
                    }
                }
            }
        }

        Cave {
            cave: cave,
        }
    }

    fn empty_cell(&self, x: i32, y: i32) -> bool {
        !self.cave.contains_key(&(x, y))
    }

    fn cave_depth(&self) -> i32 {
        self.cave.keys().map(|k| k.1).max().unwrap()
    }

    fn add_sand(&mut self) -> bool {
        let cave_depth = self.cave_depth();
        let mut x: i32 = 500;
        let mut y: i32 = 0;

        while y < cave_depth {
            if self.empty_cell(x, y + 1) {
                y += 1;
                continue;
            }

            if self.empty_cell(x - 1, y + 1) {
                x -= 1;
                y += 1;
                continue;
            }

            if self.empty_cell(x + 1, y + 1) {
                x += 1;
                y += 1;
                continue;
            }

            // println!("Sand landed at ({}, {})", x, y);
            self.cave.insert((x, y), CellType::Sand);
            return true;
        }

        false
    }

    fn add_sand2(&mut self, cave_depth: i32) -> bool {
        let mut x: i32 = 500;
        let mut y: i32 = 0;

        while y <= cave_depth {
            if self.empty_cell(x, y + 1) {
                y += 1;
                continue;
            }

            if self.empty_cell(x - 1, y + 1) {
                x -= 1;
                y += 1;
                continue;
            }

            if self.empty_cell(x + 1, y + 1) {
                x += 1;
                y += 1;
                continue;
            }

            if x == 500 && y == 0 {
                return false;
            }

            // println!("Sand landed at ({}, {})", x, y);
            self.cave.insert((x, y), CellType::Sand);
            return true;
        }

        // println!("Sand landed at ({}, {})", x, y);
        self.cave.insert((x, y), CellType::Sand);
        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day14_part1() {
        const INPUT: &str = include_str!("../inputs/day14.txt");
        let mut cave = Cave::new(INPUT.lines());

        let mut count = 0;
        while cave.add_sand() {
            count += 1;
        }

        println!("{}", count);
    }

    #[test]
    fn day14_part2() {
        const INPUT: &str = include_str!("../inputs/day14.txt");
        let mut cave = Cave::new(INPUT.lines());

        let mut count = 0;
        let cave_depth = cave.cave_depth();

        while cave.add_sand2(cave_depth) {
            count += 1;
        }

        println!("{}", count + 1);
    }
}
