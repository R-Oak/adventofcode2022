use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    is_dir: bool,
    name: String,

    size: i64,

    pub parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_file(size: i64, name: &str) -> Node {
        Node{
            is_dir: false,
            size: size,
            name: name.to_string(),

            parent: None,
            children: Vec::new(),
        }
    }

    fn new_dir(name: &str) -> Node {
        Node{
            is_dir: true,
            size: 0,
            name: name.to_string(),

            parent: None,
            children: Vec::new(),
        }
    }

    fn size(&self) -> i64 {
        match self.is_dir {
            true => {
                self.children.iter().map(|node| node.borrow().size()).sum()
            }
            false => self.size
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }

    fn get_child(&mut self, name: &str) -> Rc<RefCell<Node>> {
        if self.is_dir == false {
            panic!("get_child called on File");
        }

        for child in &self.children {
            if child.borrow().is_dir() && child.borrow().name() == name {
                return Rc::clone(&child);
            }
        }

        let child = Rc::new(RefCell::new(Node::new_dir(name)));
        self.children.push(Rc::clone(&child));
        child
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(Rc::clone(&child));
    }

    fn part1(&self) -> i64 {
        let child_sum: i64 = self.children.iter()
            .filter(|n| n.borrow().is_dir())
            .map(|n| n.borrow().part1())
            .sum();

        let size = self.size();
        if size <= 100000 {
            // println!("{} {} {}", self.name, size + child_sum, size);
            size + child_sum
        } else {
            // println!("{} {}", self.name, child_sum);
            child_sum
        }
    }

    fn part2a(&self, space_required: i64) -> Option<i64> {
        if self.is_dir {
            let size = self.size();
            if size >= space_required {
                Some(size)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn part2(&self, space_required: i64) -> Vec<i64> {
        let mut result = match self.part2a(space_required) {
            Some(size) => vec![size],
            None => vec![],
        };

        self.children.iter().for_each(|child| result.append(&mut child.borrow().part2(space_required)));

        result
    }
}

fn console(lines: core::str::Lines) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new_dir("/")));
    let mut current = Rc::clone(&root);

    for line in lines {
        if line.starts_with("$ cd") {
            match line.split(" ").nth(2) {
                Some("/") => (),
                Some("..") => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                Some(dir_name) => {
                    let child = current.borrow_mut().get_child(dir_name);
                    {
                        let mut mut_child = child.borrow_mut();
                        mut_child.parent = Some(Rc::clone(&current));
                    }
                    current = child;
                },
                None => panic!("Missing dir name")
            }
        } else if line.starts_with("$ ls") {
            ()
        } else if line.starts_with("dir") {
            // ignore directories in the listing
            ()
        } else if let Some((size, name)) = line.split_once(' ') {
            let size = size.parse::<i64>().unwrap();
            let file = Rc::new(RefCell::new(Node::new_file(size, name)));
            current.borrow_mut().add_child(file);
        } else {
            panic!("Unknown line {}", line);
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        const INPUT: &str = include_str!("../inputs/day07.txt");
        let root = console(INPUT.lines());
        println!("{}", root.borrow().part1());
    }

    #[test]
    fn day07_part2() {
        const INPUT: &str = include_str!("../inputs/day07.txt");
        let root = console(INPUT.lines());

        let space_left = 70000000 - root.borrow().size();
        let space_required: i64 = 30000000 - space_left;

        let mut sizes: Vec<i64> = root.borrow().part2(space_required);

        sizes.sort();

        println!("{}", sizes[0])
    }
}
