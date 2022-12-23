
use std::collections::HashMap;

#[derive(Debug)]
enum ErrorCode {
    HumanMonkey
}

trait Monkey {
    fn shout(&self, monkeys: &HashMap<String, Box<dyn Monkey>>) -> Result<i64, ErrorCode>;
    fn want(&self, _val: i64, monkeys: &HashMap<String, Box<dyn Monkey>>);
}

struct NumberMonkey {
    num: i64,
}

impl NumberMonkey {
    fn new(num: i64) -> NumberMonkey {
        NumberMonkey { num }
    }
}

impl Monkey for NumberMonkey {
    fn shout(&self, _monkeys: &HashMap<String, Box<dyn Monkey>>) -> Result<i64, ErrorCode> {
        Ok(self.num)
    }

    fn want(&self, val: i64, _monkeys: &HashMap<String, Box<dyn Monkey>>) {
        if self.num != val {
            panic!("Number monkey: {} != {}", self.num, val);
        }
    }
}

enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct MathMonkey {
    left: String,
    right: String,
    op: MathOp,
}

impl MathMonkey {
    fn new(left: &str, right: &str, op: MathOp) -> MathMonkey {
        MathMonkey { left: left.to_string(), right: right.to_string(), op }
    }
}

impl Monkey for MathMonkey {
    fn shout(&self, monkeys: &HashMap<String, Box<dyn Monkey>>) -> Result<i64, ErrorCode> {
        let left: i64 = monkeys[&self.left].shout(monkeys)?;
        let right: i64 = monkeys[&self.right].shout(monkeys)?;

        match self.op {
            MathOp::Add => Ok(left + right),
            MathOp::Subtract => Ok(left - right),
            MathOp::Multiply => Ok(left * right),
            MathOp::Divide => Ok(left / right)
        }
    }

    fn want(&self, val: i64, monkeys: &HashMap<String, Box<dyn Monkey>>) {
        // println!("{} {}", self.left, self.right);
        match monkeys[&self.left].shout(monkeys) {
            Ok(left_val) => {
                match self.op {
                    MathOp::Add => monkeys[&self.right].want(val - left_val, monkeys),
                    MathOp::Subtract => monkeys[&self.right].want(left_val - val, monkeys),
                    MathOp::Multiply => monkeys[&self.right].want(val / left_val, monkeys),
                    MathOp::Divide => monkeys[&self.right].want(val * left_val, monkeys),
                }
            },
            Err(_e) => {
                let right_val = monkeys[&self.right].shout(monkeys).unwrap();
                match self.op {
                    MathOp::Add => monkeys[&self.left].want(val - right_val, monkeys),
                    MathOp::Subtract => monkeys[&self.left].want(val + right_val, monkeys),
                    MathOp::Multiply => monkeys[&self.left].want(val / right_val, monkeys),
                    MathOp::Divide => monkeys[&self.left].want(val * right_val, monkeys),
                }
            }
        }
    }
}

struct HumanMonkey {

}

impl HumanMonkey {
    fn new() -> HumanMonkey {
        HumanMonkey {  }
    }
}

impl Monkey for HumanMonkey {
    fn shout(&self, _monkeys: &HashMap<String, Box<dyn Monkey>>) -> Result<i64, ErrorCode> {
        Err(ErrorCode::HumanMonkey)
    }

    fn want(&self, val: i64, _monkeys: &HashMap<String, Box<dyn Monkey>>) {
        println!("Human shouts {}", val);
    }
}

struct RootMonkey {
    left: String,
    right: String,
}

impl RootMonkey {
    fn new(left: &str, right: &str) -> RootMonkey {
        RootMonkey { left: left.to_string(), right: right.to_string() }
    }
}

impl Monkey for RootMonkey {
    fn shout(&self, monkeys: &HashMap<String, Box<dyn Monkey>>) -> Result<i64, ErrorCode> {
        panic!("Root monkey shouting")
    }

    fn want(&self, _val: i64, monkeys: &HashMap<String, Box<dyn Monkey>>) {
        match monkeys[&self.left].shout(monkeys) {
            Ok(val) => {
                monkeys[&self.right].want(val, monkeys);
                monkeys[&self.left].want(val, monkeys);
            },
            Err(_e) => {
                let val = monkeys[&self.right].shout(monkeys).unwrap();
                monkeys[&self.left].want(val, monkeys);
                monkeys[&self.right].want(val, monkeys);
            }
        }
    }
}

fn parse_line(line: &str) -> (String, Box<dyn Monkey>) {
    let words: Vec<&str> = line.split(&[' ', ':']).collect();
    match words.len() {
        3 => (
            words[0].to_string(),
            Box::new(NumberMonkey::new(
                words[2].parse::<i64>().unwrap()
            ))
        ),
        5 => (
            words[0].to_string(),
            Box::new(MathMonkey::new(
                words[2],
                words[4],
                match words[3] {
                    "+" => MathOp::Add,
                    "-" => MathOp::Subtract,
                    "*" => MathOp::Multiply,
                    "/" => MathOp::Divide,
                    _ => panic!("Unknown mathop {}", words[3])
                }
            ))
        ),
        _ => panic!("Unhandled line {}", line)
    }
}

fn parse(lines: core::str::Lines) -> HashMap<String, Box<dyn Monkey>> {
    let mut result: HashMap<String, Box<dyn Monkey>> = HashMap::<String, Box<dyn Monkey>>::new();

    for line in lines {
        let (name, monkey) = parse_line(line);
        result.insert(name, monkey);
    }

    result
}

fn parse_line2(line: &str) -> (String, Box<dyn Monkey>) {
    let words: Vec<&str> = line.split(&[' ', ':']).collect();
    match words.len() {
        3 => match words[0] {
            "humn" => (
                words[0].to_string(),
                Box::new(HumanMonkey::new())
            ),
            _ => (
                words[0].to_string(),
                Box::new(NumberMonkey::new(
                    words[2].parse::<i64>().unwrap()
                ))
            )
        },
        5 => match words[0] {
            "root" => (
                words[0].to_string(),
                Box::new(RootMonkey::new(
                    words[2],
                    words[4],
                ))
            ),
            _ => (
                words[0].to_string(),
                Box::new(MathMonkey::new(
                    words[2],
                    words[4],
                    match words[3] {
                        "+" => MathOp::Add,
                        "-" => MathOp::Subtract,
                        "*" => MathOp::Multiply,
                        "/" => MathOp::Divide,
                        _ => panic!("Unknown mathop {}", words[3])
                    }
                ))
            )
        },
        _ => panic!("Unhandled line {}", line)
    }
}

fn parse2(lines: core::str::Lines) -> HashMap<String, Box<dyn Monkey>> {
    let mut result: HashMap<String, Box<dyn Monkey>> = HashMap::<String, Box<dyn Monkey>>::new();

    for line in lines {
        let (name, monkey) = parse_line2(line);
        result.insert(name, monkey);
    }

    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day21_part1() {
        const INPUT: &str = include_str!("../inputs/day21.txt");
        let monkeys = parse(INPUT.lines());

        println!("{}", monkeys["root"].shout(&monkeys).unwrap());
    }

    #[test]
    fn day21_part2() {
        const INPUT: &str = include_str!("../inputs/day21.txt");
        let monkeys = parse2(INPUT.lines());

        monkeys["root"].want(0, &monkeys);
    }
}
