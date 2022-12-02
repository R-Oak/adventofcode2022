
enum Move {
    Rock,
    Paper,
    Scissors,
}
enum Result {
    Win,
    Lose,
    Draw,
}

fn shape_score(m: &Move) -> i64 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn result(m1: &Move, m2: &Move) -> Result {
    match m1 {
        Move::Rock => {
            match m2 {
                Move::Rock => Result::Draw,
                Move::Paper => Result::Lose,
                Move::Scissors => Result::Win,
            }
        },
        Move::Paper => {
            match m2 {
                Move::Rock => Result::Win,
                Move::Paper => Result::Draw,
                Move::Scissors => Result::Lose,
            }
        },
        Move::Scissors => {
            match m2 {
                Move::Rock => Result::Lose,
                Move::Paper => Result::Win,
                Move::Scissors => Result::Draw,
            }
        },
    }
}

fn get_move(m: &Move, r: &Result) -> Move {
    match m {
        Move::Rock => {
            match r {
                Result::Win => Move::Paper,
                Result::Lose => Move::Scissors,
                Result::Draw => Move::Rock,
            }
        },
        Move::Paper => {
            match r {
                Result::Win => Move::Scissors,
                Result::Lose => Move::Rock,
                Result::Draw => Move::Paper,
            }
        },
        Move::Scissors => {
            match r {
                Result::Win => Move::Rock,
                Result::Lose => Move::Paper,
                Result::Draw => Move::Scissors,
            }
        },
    }
}

fn tournament(lines: core::str::Lines) -> i64 {
    let mut score: i64 = 0;

    for line in lines {
        let opponent_move: Move = match line.chars().nth(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Unknown move")
        };

        let my_move: Move = match line.chars().nth(2).unwrap() {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Unknown move")
        };

        score += shape_score(&my_move);
        score += match result(&my_move, &opponent_move) {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Draw => 3,
        }
    }

    score
}

fn tournament2(lines: core::str::Lines) -> i64 {
    let mut score: i64 = 0;

    for line in lines {
        let opponent_move: Move = match line.chars().nth(0).unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Unknown move")
        };

        let desired_result: Result = match line.chars().nth(2).unwrap() {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Unknown result")
        };

        let my_move: Move = get_move(&opponent_move, &desired_result);

        score += shape_score(&my_move);
        score += match result(&my_move, &opponent_move) {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Draw => 3,
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1() {
        const INPUT: &str = include_str!("../inputs/day02.txt");
        let score: i64 = tournament(INPUT.lines());

        println!("{}", score);
    }

    #[test]
    fn day02_par21() {
        const INPUT: &str = include_str!("../inputs/day02.txt");
        let score: i64 = tournament2(INPUT.lines());

        println!("{}", score);
    }
}
