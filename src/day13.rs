
use std::cmp::Ordering;

enum Type {
    Integer,
    List,
}

struct Elem {
    pub elem_type: Type,

    pub value: i32,
    pub list: Vec<Elem>,
}

struct ElemParser {
    string: String,
    index: usize,
}

impl ElemParser {
    pub fn parse(string: &str) -> Elem {
        let mut parser = ElemParser {
            string: string.to_string(),
            index: 0
        };

        parser.parse_elem()
    }

    fn peek(&self) -> u8 {
        match self.string.bytes().nth(self.index) {
            Some(c) => c,
            None => panic!("String has been consumed")
        }
    }

    fn next(&mut self) -> u8 {
        let result = match self.string.bytes().nth(self.index) {
            Some(c) => c,
            None => panic!("String has been consumed")
        };

        self.index += 1;

        result
    }

    fn take(&mut self, expected: u8) {
        if self.peek() != expected {
            panic!("{} expected - {} found", expected, self.peek());
        }

        self.next();
    }

    fn parse_integer(&mut self) -> Elem {
        let mut value: i32 = 0;

        loop {
            let c = self.peek();
            match c {
                b'0'..=b'9' => {
                    let c = self.next();
                    value *= 10;
                    value += (c - b'0') as i32;
                }
                _ => break,
            }
        }

        Elem { elem_type: Type::Integer, value: value, list: vec![] }
    }

    fn parse_list(&mut self) -> Elem {
        let mut children = vec![];

        self.take(b'[');

        loop {
            match self.peek() {
                b']' => break,
                _ => {
                    children.push(self.parse_elem());
                    if self.peek() == b',' {
                        self.take(b',');
                    }
                }
            }
        }

        self.take(b']');

        Elem { elem_type: Type::List, value: 0, list: children }
    }

    fn parse_elem(&mut self) -> Elem {
        let c = self.peek();
        match c {
            b'[' => self.parse_list(),
            b'0'..=b'9' => self.parse_integer(),
            _ => panic!("Unknown character {}", c),
        }
    }
}

fn in_order_lists(elem1: &Elem, elem2: &Elem) -> Ordering {
    let mut idx = 0;

    loop {
        match (elem1.list.iter().nth(idx), elem2.list.iter().nth(idx)) {
            (Some(child1), Some(child2)) => {
                match in_order(child1, child2) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                }
            }
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
        }
        idx += 1;
    }
}


fn in_order(elem1: &Elem, elem2: &Elem) -> Ordering {
    match (&elem1.elem_type, &elem2.elem_type) {
        (Type::Integer, Type::Integer) => {
            if elem1.value < elem2.value {
                Ordering::Less
            } else if elem1.value == elem2.value {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        (Type::List, Type::Integer) => {
            let new_child = Elem { elem_type: Type::Integer, value: elem2.value, list: vec![] };
            let new_elem = Elem { elem_type: Type::List, value: 0, list: vec![new_child] };
            in_order_lists(elem1, &new_elem)
        }
        (Type::Integer, Type::List) => {
            let new_child = Elem { elem_type: Type::Integer, value: elem1.value, list: vec![] };
            let new_elem = Elem { elem_type: Type::List, value: 0, list: vec![new_child] };
            in_order_lists(&new_elem, elem2)
        },
        (Type::List, Type::List) => in_order_lists(elem1, elem2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1() {
        const INPUT: &str = include_str!("../inputs/day13.txt");
        let mut lines = INPUT.lines();
        let mut idx: i32 = 1;
        let mut result: i32 = 0;

        while let (Some(line1), Some(line2)) = (lines.next(), lines.next()) {
            let left = ElemParser::parse(line1);
            let right = ElemParser::parse(line2);

            match in_order(&left, &right) {
                Ordering::Less => {
                    // println!("{} {}", line1, line2);
                    result += idx;
                }
                _ => (),
            }
            idx += 1;

            // read the blank line
            lines.next();
        }

        println!("{}", result);
    }

    #[test]
    fn day13_part2() {
        const INPUT: &str = include_str!("../inputs/day13.txt");
        let mut lines = INPUT.lines();
        let mut vec = vec![
            ElemParser::parse("[[2]]"),
            ElemParser::parse("[[6]]"),
        ];

        while let (Some(line1), Some(line2)) = (lines.next(), lines.next()) {
            vec.push(ElemParser::parse(line1));
            vec.push(ElemParser::parse(line2));

            // read the blank line
            lines.next();
        }

        vec.sort_by(|a, b| in_order(a, b));

        let mut result = 1;
        let marker1 = ElemParser::parse("[[2]]");
        let marker2 = ElemParser::parse("[[6]]");

        for (idx, elem) in vec.iter().enumerate() {
            if in_order(elem, &marker1) == Ordering::Equal {
                result *= idx + 1;
            }
            if in_order(elem, &marker2) == Ordering::Equal {
                result *= idx + 1;
            }
        }

        println!("{}", result);
    }
}
