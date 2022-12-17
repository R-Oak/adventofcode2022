
use std::collections::HashMap;
use std::collections::HashSet;

struct Cave {
    cave: HashSet::<(i32, i32)>,
    jets: String,
    jet_index: usize,
}

impl Cave {
    fn new(jets: &str) -> Cave {
        Cave {
            cave: HashSet::<(i32, i32)>::new(),
            jets: jets.trim().to_string(),
            jet_index: 0,
        }
    }

    fn height(&self) -> i32 {
        match self.cave.iter().map(|val| val.1).max() {
            Some(height) => height,
            None => 0,
        }
    }

    fn empty(&self, cell: &(i32, i32)) -> bool {
        const CAVE_WIDTH: i32 = 7;

        if cell.0 < 0 || cell.0 >= CAVE_WIDTH {
            return false;
        }
        if cell.1 <= 0 {
            return false;
        }
        !self.cave.contains(cell)
    }

    fn jet(&mut self) -> char {
        if let Some(result) = self.jets.chars().nth(self.jet_index) {
            self.jet_index += 1;
            self.jet_index %= self.jets.len();
            result
        } else {
            panic!("Bad jet_index")
        }
    }

    fn add_rock(&mut self, rock: &Rock) -> i32 {
        let mut rock_x: i32 = 2;
        let mut rock_y: i32 = self.height() + 4;

        loop {
            match self.jet() {
                '<' => if rock.can_left(self, rock_x, rock_y) { rock_x -= 1 }
                '>' => if rock.can_right(self, rock_x, rock_y) { rock_x += 1 }
                _ => panic!("Unknown jet")
            }

            match rock.can_fall(self, rock_x, rock_y) {
                true => { rock_y -= 1 }
                false => break
            }
        }

        // println!("Rock added at ({},{})", rock_x, rock_y);
        for val in &rock.rock {
            self.cave.insert((rock_x + val.0, rock_y + val.1));
        }

        rock_x
    }
}

struct Rock {
    rock: HashSet::<(i32, i32)>,
}

impl Rock {
    fn rocks() -> Vec<Rock> {
        vec![
            vec![ (0, 0), (1, 0), (2, 0), (3, 0) ],
            vec![ (1, 0), (0, 1), (1, 1), (2, 1), (1, 2) ],
            vec![ (0, 0), (1, 0), (2, 0), (2, 1), (2, 2) ],
            vec![ (0, 0), (0, 1), (0, 2), (0, 3) ],
            vec![ (0, 0), (1, 0), (0, 1), (1, 1) ],
        ].iter().map(|data| Rock::new(&data.to_vec())).collect()
    }

    fn new(data: &Vec::<(i32, i32)>) -> Rock {
        let mut rock = HashSet::<(i32, i32)>::new();

        for value in data {
            rock.insert(value.clone());
        }

        Rock { rock }
    }

    fn width(&self) -> i32 {
        self.rock.iter().map(|val| val.0).max().unwrap() + 1
    }

    fn can_left(&self, cave: &Cave, rock_x: i32, rock_y: i32) -> bool {
        self.rock.iter().all(|val| cave.empty(&(rock_x + val.0 - 1, rock_y + val.1)))
    }

    fn can_right(&self, cave: &Cave, rock_x: i32, rock_y: i32) -> bool {
        self.rock.iter().all(|val| cave.empty(&(rock_x + val.0 + 1, rock_y + val.1)))
    }

    fn can_fall(&self, cave: &Cave, rock_x: i32, rock_y: i32) -> bool {
        self.rock.iter().all(|val| cave.empty(&(rock_x + val.0, rock_y + val.1 - 1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_part1() {
        const INPUT: &str = include_str!("../inputs/day17.txt");
        let mut cave = Cave::new(INPUT);
        let rocks = Rock::rocks();
        let mut idx: usize = 0;

        for _ in 0..2022 {
            cave.add_rock(&rocks[idx]);
            idx += 1;
            idx %= rocks.len();
        }

        println!("{}", cave.height());
    }

    #[test]
    fn day17_part2() {
        const INPUT: &str = include_str!("../inputs/day17.txt");
        let mut cave = Cave::new(INPUT);
        let rocks = Rock::rocks();
        let mut idx: usize = 0;
        let mut map: HashMap::<(i32, usize), (i64, i64)> = HashMap::<(i32, usize),(i64, i64)>::new();
        let mut rock_num = 0;

        loop {
            let height = cave.height() as i64;
            let rock_x = cave.add_rock(&rocks[idx]);

            if idx == 0 {
                //println!("{} {} {} {}", rock_x, cave.jet_index, height, rock_num);
                if let Some((h, r)) = map.get(&(rock_x, cave.jet_index)) {
                    let cycle_height = height - h;
                    let cycle_len = rock_num - r;
                    let num_rocks: i64 = 1000000000000 - r;

                    let cycles = num_rocks / cycle_len;
                    let extra = num_rocks % cycle_len;

                    let extra_height : i64 = map.values().filter(|v| v.1 == extra).map(|v| v.0).sum();

                    println!("{}", cycles * cycle_height + h + extra_height);

                    break;
                } else {
                    map.insert((rock_x, cave.jet_index), (height, rock_num));
                }
            }

            idx += 1;
            idx %= rocks.len();
            rock_num += 1;
        }

    }
}
