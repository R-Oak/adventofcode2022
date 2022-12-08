
fn parse(lines: core::str::Lines) -> Vec<Vec<u8>> {
    lines.map(|line| line.bytes().map(|c| c - b'0').collect()).collect()
}

fn tree_visible(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> bool {
    let h: usize = grid.len();
    let w: usize = grid[0].len();
    let mut visible = true;

    for i in 0..x {
        if grid[i][y] >= grid[x][y] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for i in x+1..w {
        if grid[i][y] >= grid[x][y] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for j in 0..y {
        if grid[x][j] >= grid[x][y] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for j in y+1..h {
        if grid[x][j] >= grid[x][y] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    false
}

fn scenic_score(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> i64 {
    let h: usize = grid.len();
    let w: usize = grid[0].len();
    let mut result: i64 = 1;
    let mut dist = 0;

    for i in (0..x).rev() {
        dist += 1;
        if grid[i][y] >= grid[x][y] {
            break;
        }
    }

    result *= dist;
    dist = 0;

    for i in x+1..w {
        dist += 1;
        if grid[i][y] >= grid[x][y] {
            break;
        }
    }

    result *= dist;
    dist = 0;

    for j in (0..y).rev() {
        dist += 1;
        if grid[x][j] >= grid[x][y] {
            break;
        }
    }

    result *= dist;
    dist = 0;

    for j in y+1..h {
        dist += 1;
        if grid[x][j] >= grid[x][y] {
            break;
        }
    }

    result *= dist;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1() {
        const INPUT: &str = include_str!("../inputs/day08.txt");

        let grid = parse(INPUT.lines());
        let mut total = 0;
        let h: usize = grid.len();
        let w: usize = grid[0].len();

        for x in 0..w {
            for y in 0..h {
                if tree_visible(x, y, &grid) {
                    total += 1;
                }
            }
        }

        println!("{}", total);
    }

    #[test]
    fn day08_part2() {
        const INPUT: &str = include_str!("../inputs/day08.txt");

        let grid = parse(INPUT.lines());

        let best = grid.iter().enumerate().map(|(x, _line)| grid[x].iter().enumerate().map(|(y, _tree)| scenic_score(x, y, &grid)).max().unwrap()).max().unwrap();

        println!("{}", best);
    }
}
