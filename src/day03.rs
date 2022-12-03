
fn rucksacks(lines: core::str::Lines) -> i64 {
    let mut result: i64 = 0;

    for line in lines {
        let common: u8 = rucksack(line);

        result += score(common) as i64;
    }

    result
}

fn rucksack(line: &str) -> u8 {
    let len = line.len();
    let bag1 = &line[0..len/2];
    let bag2 = &line[len/2..len];

    for c1 in bag1.bytes() {
        for c2 in bag2.bytes() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    panic!("Didn't find the common character")
}

fn score(c: u8) -> u8 {
    // 97 = 'a'
    // 65 = 'A'
    if c < 97  {
        (c - 65) + 27
    } else {
        (c - 97) + 1
    }
}

fn rucksacks2(lines: &mut core::str::Lines) -> i64 {
    let mut result: i64 = 0;

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let common = rucksack2(line1, line2, line3);

        result += score(common) as i64;
    }

    result
}

fn rucksack2(line1: &str, line2: &str, line3: &str) -> u8 {
    for c1 in line1.bytes() {
        let mut common = 0;

        for c2 in line2.bytes() {
            if c1 == c2 {
                common = c1;
                break;
            }
        }

        if common == 0 {
            continue;
        }

        for c3 in line3.bytes() {
            if common == c3 {
                return c1;
            }
        }
    }

    panic!("Didn't find the common character")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1() {
        const INPUT: &str = include_str!("../inputs/day03.txt");
        let result: i64 = rucksacks(INPUT.lines());

        println!("{}", result);
    }

    #[test]
    fn day03_part2() {
        const INPUT: &str = include_str!("../inputs/day03.txt");
        let result: i64 = rucksacks2(&mut INPUT.lines());

        println!("{}", result);
    }
}
