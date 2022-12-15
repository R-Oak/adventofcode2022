
use std::ops::Range;
use std::collections::HashSet;

struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    fn new(sensor_pos: (i32, i32), beacon_pos: (i32, i32)) -> Sensor {
        Sensor {
            sensor_x: sensor_pos.0,
            sensor_y: sensor_pos.1,
            beacon_x: beacon_pos.0,
            beacon_y: beacon_pos.1,
        }
    }

    fn radius(&self) -> i32 {
        (self.sensor_x - self.beacon_x).abs() + (self.sensor_y - self.beacon_y).abs()
    }

    fn intersection(&self, row: i32) -> Option<Range<i32>> {
        let radius: i32 = self.radius();
        let vert = (self.sensor_y - row).abs();
        let horz = radius - vert;

        if horz < 0 {
            None
        } else {
            Some((self.sensor_x - horz)..(self.sensor_x + horz))
        }
    }
}

fn parse_equal(line: &str) -> i32 {
    if let Some((_name, val)) = line.split_once('=') {
        val.parse::<i32>().unwrap()
    } else {
        panic!("Missing '=' in {}", line);
    }
}

fn parse_coord(line: &str) -> (i32, i32) {
    if let Some((x, y)) = line.split_once(", ") {
        (
            parse_equal(x),
            parse_equal(y)
        )
    } else {
        panic!("Missing ', ' in {}", line);
    }
}

fn sensor_pos(line: &str) -> (i32, i32) {
    let header = "Sensor at ";
    parse_coord(&line[header.len()..])
}

fn beacon_pos(line: &str) -> (i32, i32) {
    let header = "closest beacon is at ";
    parse_coord(&line[header.len()..])
}

fn parse_line(line: &str) -> Sensor {
    if let Some((sensor, beacon)) = line.split_once(": ") {
        Sensor::new(
            sensor_pos(sensor),
            beacon_pos(beacon)
        )
    } else {
        panic!("Missing ': ' in {}", line);
    }
}

fn parse(lines: core::str::Lines) -> Vec<Sensor> {
    lines.map(|line| parse_line(line)).collect()
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn day15_part1() {
        const INPUT: &str = include_str!("../inputs/day15.txt");
        let sensors: Vec<Sensor> = parse(INPUT.lines());
        let mut set: HashSet<i32> = HashSet::<i32>::new();

        for sensor in sensors {
            if let Some(range) = sensor.intersection(2000000) {
                for x in range {
                    set.insert(x);
                }
            }
        }

        println!("{}", set.len());
    }

    #[test]
    fn day15_part2() {
        const INPUT: &str = include_str!("../inputs/day15.txt");
        let sensors: Vec<Sensor> = parse(INPUT.lines());

        for row in 0..4000000 {
            let mut x = 0;
            loop {
                let mut found = false;
                for idx in 0..sensors.len() {
                    if let Some(range) = sensors[idx].intersection(row) {
                        if range.contains(&x) {
                            x = range.end + 1;
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    if x < 4000000 {
                        println!("{} {} {}", row, x, x as i64 * 4000000 + row as i64);
                    }
                    break;
                }
            }
        }
    }
}
