
fn cpu(lines: core::str::Lines) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut result = 0;

    for line in lines {
        // println!("{} {}", cycle, line);
        if line.starts_with("addx") {
            if cycle % 40 == 19 {
                result += (cycle + 1) * x;
                println!("{} {} {}", (cycle + 1), x, (cycle + 1) * x);
            }
            cycle += 2;

            if cycle % 40 == 20 {
                result += cycle * x;
                println!("{} {} {}", cycle, x, cycle * x);
            }

            if let Some((_cmd, amt)) = line.split_once(' ') {
                x += amt.parse::<i32>().unwrap();
            }
        } else if line.starts_with("noop") {
            cycle += 1;
            if cycle % 40 == 20 {
                result += cycle * x;
                println!("{} {} {}", cycle, x, cycle * x);
            }
        } else {
            panic!("Bad line {}", line);
        }
    }

    result
}

fn print_pixel(cycle: i32, x: i32) {
    if cycle % 40 == 0 {
        print!("\n");
    }

    let horz = cycle % 40;
    if horz >= x - 1 && horz <= x + 1 {
        print!("#");
    } else {
        print!(" ");
    }
}

fn cpu2(lines: core::str::Lines) -> i32 {
    let mut cycle = 0;
    let mut x = 1;

    for line in lines {
        if line.starts_with("addx") {
            print_pixel(cycle, x);
            cycle += 1;
            print_pixel(cycle, x);
            cycle += 1;
            if let Some((_cmd, amt)) = line.split_once(' ') {
                x += amt.parse::<i32>().unwrap();
            }
        } else if line.starts_with("noop") {
            print_pixel(cycle, x);
            cycle += 1;
        } else {
            panic!("Bad line {}", line);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1() {
        const INPUT: &str = include_str!("../inputs/day10.txt");

        println!("{}", cpu(INPUT.lines()));
    }

    #[test]
    fn day10_part2() {
        const INPUT: &str = include_str!("../inputs/day10.txt");

        println!("{}", cpu2(INPUT.lines()));
    }
}
