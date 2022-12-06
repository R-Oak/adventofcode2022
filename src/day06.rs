
use std::collections::HashSet;

fn start_of_marker(str: &[u8]) -> bool {
    let mut set = HashSet::new();

    for c in str {
        if set.contains(c) {
            return false;
        }
        set.insert(c);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1() {
        const INPUT: &str = include_str!("../inputs/day06.txt");
        // const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        for (idx, str) in INPUT.as_bytes().windows(4).enumerate() {
            if start_of_marker(str) {
                    println!("{}", idx + 4);
                    break;
            }
        }
    }

    #[test]
    fn day06_part2() {
        const INPUT: &str = include_str!("../inputs/day06.txt");
        // const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        for (idx, str) in INPUT.as_bytes().windows(14).enumerate() {
            if start_of_marker(str) {
                    println!("{}", idx + 14);
                    break;
            }
        }
    }
}
