
use std::collections::HashMap;

fn parse(lines: core::str::Lines) -> HashMap::<usize, Vec::<u8>> {
    let mut result: HashMap::<usize, Vec::<u8>> = HashMap::new();

    for line in lines {
        if line.bytes().nth(1).unwrap() == 0x31 {
            break;
        }

        let mut idx: usize = 1;
        let line_len: usize = line.len();
        let mut pile: usize = 1;

        while idx < line_len {
            if result.contains_key(&pile) == false {
                result.insert(pile, Vec::<u8>::new());
            }

            if line.bytes().nth(idx).unwrap() != 32 {
                result.entry(pile).and_modify(|v| v.push(line.bytes().nth(idx).unwrap()));
            }

            pile += 1;
            idx += 4;
        }
    }

    result
}

fn print_state(state: &HashMap::<usize, Vec::<u8>>) {
    let mut idx: usize = 1;
    while state.contains_key(&idx) {
        for c in state[&idx].as_slice() {
            print!("{} ", *c as char);
        }
        print!("\n");
        idx += 1;
    }
    println!("");
}

fn run(lines: core::str::Lines, mut state: HashMap::<usize, Vec::<u8>>) -> HashMap::<usize, Vec::<u8>> {
    for line in lines {
        if line.starts_with("move") == false {
            continue;
        }

        // print_state(&state);

        let words: Vec<&str> = line.split(' ').collect();
        let num: usize = words[1].parse::<usize>().unwrap();
        let src_idx: usize = words[3].parse::<usize>().unwrap();
        let dst_idx: usize = words[5].parse::<usize>().unwrap();

        let mut src: Vec<u8> = if let Some(src) = state.remove(&src_idx) { src } else { todo!() };
        let mut dst: Vec<u8> = if let Some(dst) = state.remove(&dst_idx) { dst } else { todo!() };

        let mut vec: Vec<u8> = src.drain(0..num).rev().collect();
        vec.append(&mut dst);

        state.insert(src_idx, src);
        state.insert(dst_idx, vec);
    }

    state
}

fn run2(lines: core::str::Lines, mut state: HashMap::<usize, Vec::<u8>>) -> HashMap::<usize, Vec::<u8>> {
    for line in lines {
        if line.starts_with("move") == false {
            continue;
        }

        // print_state(&state);

        let words: Vec<&str> = line.split(' ').collect();
        let num: usize = words[1].parse::<usize>().unwrap();
        let src_idx: usize = words[3].parse::<usize>().unwrap();
        let dst_idx: usize = words[5].parse::<usize>().unwrap();

        let mut src: Vec<u8> = if let Some(src) = state.remove(&src_idx) { src } else { todo!() };
        let mut dst: Vec<u8> = if let Some(dst) = state.remove(&dst_idx) { dst } else { todo!() };

        let mut vec: Vec<u8> = src.drain(0..num).collect();
        vec.append(&mut dst);

        state.insert(src_idx, src);
        state.insert(dst_idx, vec);
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1() {
        const INPUT: &str = include_str!("../inputs/day05.txt");
        let state = run(INPUT.lines(), parse(INPUT.lines()));

        let mut idx: usize = 1;
        while state.contains_key(&idx) {
            print!("{}", state[&idx][0] as char);
            idx += 1;
        }
        println!("");
    }

    #[test]
    fn day05_part2() {
        const INPUT: &str = include_str!("../inputs/day05.txt");
        let state = run2(INPUT.lines(), parse(INPUT.lines()));

        let mut idx: usize = 1;
        while state.contains_key(&idx) {
            print!("{}", state[&idx][0] as char);
            idx += 1;
        }
        println!("");
    }
}
