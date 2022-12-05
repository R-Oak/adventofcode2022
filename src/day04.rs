
fn contains(x1: i64, x2: i64, y1: i64, y2: i64) -> bool {
    (y1 >= x1 && y1 <= x2 && y2 >= x1 && y2 <= x2) ||
    (x1 >= y1 && x1 <= y2 && x2 >= y1 && x2 <= y2)
}

fn overlaps(x1: i64, x2: i64, y1: i64, y2: i64) -> bool {
    (y1 >= x1 && y1 <= x2) || (y2 >= x1 && y2 <= x2) ||
    (x1 >= y1 && x1 <= y2) || (x2 >= y1 && x2 <= y2)
}

fn cleaning(lines: core::str::Lines) -> i64 {
    let mut result: i64 = 0;

    for line in lines {
        let sections: Vec<i64> = line.split(&[',','-'][..])
            .map(|section| section.parse::<i64>().unwrap())
            .collect();

        if contains(sections[0], sections[1], sections[2], sections[3]) {
            result += 1;
        }
    }

    result
}

fn cleaning2(lines: core::str::Lines) -> i64 {
    let mut result: i64 = 0;

    for line in lines {
        let sections: Vec<i64> = line.split(&[',','-'][..])
            .map(|section| section.parse::<i64>().unwrap())
            .collect();

        if overlaps(sections[0], sections[1], sections[2], sections[3]) {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1() {
        const INPUT: &str = include_str!("../inputs/day04.txt");
        let overlaps = cleaning(INPUT.lines());

        println!("{}", overlaps);
    }

    #[test]
    fn day04_part2() {
        const INPUT: &str = include_str!("../inputs/day04.txt");
        let overlaps = cleaning2(INPUT.lines());

        println!("{}", overlaps);
    }
}
