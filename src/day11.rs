
struct Monkey {
    pub items: Vec<i64>,
    pub operation: fn(i64) -> i64,
    pub divisor: i64,
    pub true_target: usize,
    pub false_target: usize,
    pub inspections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1() {
        // let mut monkeys:Vec<Monkey> = vec![
        //     Monkey { items: vec![79, 98], operation: |x| x * 19, divisor: 23, true_target: 2, false_target: 3, inspections: 0 },
        //     Monkey { items: vec![54, 65, 75, 74], operation: |x| x + 6, divisor: 19, true_target: 2, false_target: 0, inspections: 0 },
        //     Monkey { items: vec![79, 60, 97], operation: |x| x * x, divisor: 13, true_target: 1, false_target: 3, inspections: 0 },
        //     Monkey { items: vec![74], operation: |x| x + 3, divisor: 17, true_target: 0, false_target: 1, inspections: 0 },
        // ];
        let mut monkeys:Vec<Monkey> = vec![
            Monkey { items: vec![73, 77], operation: |x| x * 5, divisor: 11, true_target: 6, false_target: 5, inspections: 0 },
            Monkey { items: vec![57, 88, 80], operation: |x| x + 5, divisor: 19, true_target: 6, false_target: 0, inspections: 0 },
            Monkey { items: vec![61, 81, 84, 69, 77, 88], operation: |x| x * 19, divisor: 5, true_target: 3, false_target: 1, inspections: 0 },
            Monkey { items: vec![78, 89, 71, 60, 81, 84, 87, 75], operation: |x| x + 7, divisor: 3, true_target: 1, false_target: 0, inspections: 0 },
            Monkey { items: vec![60, 76, 90, 63, 86, 87, 89], operation: |x| x + 2, divisor: 13, true_target: 2, false_target: 7, inspections: 0 },
            Monkey { items: vec![88], operation: |x| x + 1, divisor: 17, true_target: 4, false_target: 7, inspections: 0 },
            Monkey { items: vec![84, 98, 78, 85], operation: |x| x * x, divisor: 7, true_target: 5, false_target: 4, inspections: 0 },
            Monkey { items: vec![98, 89, 78, 73, 71], operation: |x| x + 4, divisor: 2, true_target: 3, false_target: 2, inspections: 0 },
        ];

        for _ in 0..20 {
            let mut idx = 0;

            while idx < monkeys.len() {
                monkeys[idx].inspections += monkeys[idx].items.len();

                for val_idx in 0..monkeys[idx].items.len() {
                    let val = (monkeys[idx].operation)(monkeys[idx].items[val_idx]) / 3;
                    let target = if val % monkeys[idx].divisor == 0 {
                        monkeys[idx].true_target
                    } else {
                        monkeys[idx].false_target
                    };
                    monkeys[target].items.push(val);
                }
                monkeys[idx].items = vec![];
                idx += 1;
            }
        }

        let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
        inspections.sort_by(|a, b| b.cmp(a));
        println!("{:?}", inspections[0] * inspections[1]);
    }

    #[test]
    fn day11_part2() {
        // let mut monkeys:Vec<Monkey> = vec![
        //     Monkey { items: vec![79, 98], operation: |x| x * 19, divisor: 23, true_target: 2, false_target: 3, inspections: 0 },
        //     Monkey { items: vec![54, 65, 75, 74], operation: |x| x + 6, divisor: 19, true_target: 2, false_target: 0, inspections: 0 },
        //     Monkey { items: vec![79, 60, 97], operation: |x| x * x, divisor: 13, true_target: 1, false_target: 3, inspections: 0 },
        //     Monkey { items: vec![74], operation: |x| x + 3, divisor: 17, true_target: 0, false_target: 1, inspections: 0 },
        // ];
        let mut monkeys:Vec<Monkey> = vec![
            Monkey { items: vec![73, 77], operation: |x| x * 5, divisor: 11, true_target: 6, false_target: 5, inspections: 0 },
            Monkey { items: vec![57, 88, 80], operation: |x| x + 5, divisor: 19, true_target: 6, false_target: 0, inspections: 0 },
            Monkey { items: vec![61, 81, 84, 69, 77, 88], operation: |x| x * 19, divisor: 5, true_target: 3, false_target: 1, inspections: 0 },
            Monkey { items: vec![78, 89, 71, 60, 81, 84, 87, 75], operation: |x| x + 7, divisor: 3, true_target: 1, false_target: 0, inspections: 0 },
            Monkey { items: vec![60, 76, 90, 63, 86, 87, 89], operation: |x| x + 2, divisor: 13, true_target: 2, false_target: 7, inspections: 0 },
            Monkey { items: vec![88], operation: |x| x + 1, divisor: 17, true_target: 4, false_target: 7, inspections: 0 },
            Monkey { items: vec![84, 98, 78, 85], operation: |x| x * x, divisor: 7, true_target: 5, false_target: 4, inspections: 0 },
            Monkey { items: vec![98, 89, 78, 73, 71], operation: |x| x + 4, divisor: 2, true_target: 3, false_target: 2, inspections: 0 },
        ];

        let mut divisor = 1;

        for monkey in &monkeys {
            divisor *= monkey.divisor;
        }

        for _ in 0..10000 {
            let mut idx = 0;

            while idx < monkeys.len() {
                monkeys[idx].inspections += monkeys[idx].items.len();

                for val_idx in 0..monkeys[idx].items.len() {
                    let val = (monkeys[idx].operation)(monkeys[idx].items[val_idx]);
                    let target = if val % monkeys[idx].divisor == 0 {
                        monkeys[idx].true_target
                    } else {
                        monkeys[idx].false_target
                    };
                    monkeys[target].items.push(val % divisor);
                }
                monkeys[idx].items = vec![];
                idx += 1;
            }
        }

        let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
        inspections.sort_by(|a, b| b.cmp(a));
        println!("{:?}", inspections[0] * inspections[1]);
    }
}
