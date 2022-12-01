
fn elves(lines: core::str::Lines) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut total: i64 = 0;

    for line in lines {
        if line == "" {
            result.push(total);
            total = 0;
        } else {
            let val: i64 = line.parse().unwrap();
            total += val;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1() {
        const INPUT: &str = include_str!("../inputs/day01.txt");
        let elves = elves(INPUT.lines());

        println!("{}", elves.iter().max().unwrap());
    }

    #[test]
    fn day01_part2() {
        const INPUT: &str = include_str!("../inputs/day01.txt");
        let mut elves = elves(INPUT.lines());

        elves.sort_by(|a, b| b.cmp(a));

        let total: i64 = elves[0..3].iter().sum();

        println!("{}", total);
    }
}
