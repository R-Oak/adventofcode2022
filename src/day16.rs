
use std::collections::{HashMap, LinkedList};

struct Valve {
    name: String,
    flow: i32,
    next: Vec<String>,
    index: i64,
}

impl Valve {
    fn new(line: &str) -> Valve {
        if let Some((name, next)) = line.split_once("; ") {
            let name_parts: Vec<&str> = name.split(" ").collect();
            let mut flow: i32 = 0;
            if let Some((_, flow_str)) = name_parts[4].split_once('=') {
                flow = flow_str.parse::<i32>().unwrap();
            } else {
                panic!("Bad flow {}", name_parts[4])
            }

            let header: &str = "tunnels lead to valves ";
            let valves: &str = if next.starts_with(header) {
                &next[header.len()..]
            } else {
                &next[header.len() - 1..]
            };

            Valve {
                name: name_parts[1].to_string(),
                flow,
                next: valves.split(", ").map(|s| s.to_string()).collect(),
                index: 0
            }
        } else {
            panic!("Bad line {}", line)
        }
    }
}

fn parse(lines: core::str::Lines) -> HashMap<String, Valve> {
    let mut map: HashMap<String, Valve> = HashMap::new();
    let mut idx: i64 = 0x01;

    for line in lines {
        let mut valve = Valve::new(line);
        valve.index = idx;
        idx *= 2;
        let name = String::from(&valve.name);

        map.insert(name, valve);
    }

    map
}

fn compute_distances(valves: &HashMap<String, Valve>) -> HashMap<(String, String),i32> {
    let mut map: HashMap::<(String, String),i32> = HashMap::new();

    for k1 in valves.keys() {
        for k2 in valves.keys() {
            map.insert((k1.to_string(), k2.to_string()), 999);
        }
    }

    for valve in valves.values() {
        map.insert((valve.name.to_string(), valve.name.to_string()), 0);

        let mut worklist: LinkedList<&str> = LinkedList::<&str>::new();
        worklist.push_back(&valve.name);

        while let Some(v) = worklist.pop_front() {
            let d = map[&(valve.name.to_string(), v.to_string())];

            for n in &valves[&v.to_string()].next {
                let cur = map[&(valve.name.to_string(), n.to_string())];
                if cur > d + 1 {
                    map.insert((valve.name.to_string(), n.to_string()), d + 1);
                    worklist.push_back(n);
                }
            }
        }
    }

    map
}

struct State {
    current: String,
    valves_open: i64,
    minutes_left: i32,
    total_flow: i32,
}

impl State {
    fn new(valves: &HashMap<String, Valve>) -> State {
        let zero_flow = valves.values().filter(|v| v.flow == 0).map(|v| v.index).sum();

        State {
            current: "AA".to_string(),
            valves_open: zero_flow,
            minutes_left: 30,
            total_flow: 0,
        }
    }

    fn next(&self, valves: &HashMap<String, Valve>, distances: &HashMap<(String, String), i32>) -> Vec<State> {
        valves.values()
            .filter(|v| self.valves_open & v.index == 0)
            .filter(|v| distances[&(self.current.to_string(), v.name.to_string())] + 1 <= self.minutes_left)
            .map(|v|
                State {
                    current: v.name.to_string(),
                    valves_open: self.valves_open + v.index,
                    minutes_left: self.minutes_left - (distances[&(self.current.to_string(), v.name.to_string())] + 1),
                    total_flow: self.total_flow + ((self.minutes_left - (distances[&(self.current.to_string(), v.name.to_string())] + 1)) * v.flow)
                }
            ).collect()
    }
}

struct StateWithElephant {
    current: Vec<String>,
    valves_open: i64,
    minutes_left: Vec<i32>,
    total_flow: i32,
}

impl StateWithElephant {
    fn new(valves: &HashMap<String, Valve>) -> StateWithElephant {
        let zero_flow = valves.values().filter(|v| v.flow == 0).map(|v| v.index).sum();

        StateWithElephant {
            current: vec!["AA".to_string(), "AA".to_string()],
            valves_open: zero_flow,
            minutes_left: vec![26, 26],
            total_flow: 0,
        }
    }

    fn next_index(&self, idx: usize, valves: &HashMap<String, Valve>, distances: &HashMap<(String, String), i32>) -> Vec<StateWithElephant> {
        valves.values()
            .filter(|v| self.valves_open & v.index == 0)
            .filter(|v| distances[&(self.current[idx].to_string(), v.name.to_string())] + 1 <= self.minutes_left[idx])
            .map(|v|
                StateWithElephant {
                    current: if idx == 0 {
                        vec![v.name.to_string(), self.current[1].to_string()]
                    } else {
                        vec![self.current[0].to_string(), v.name.to_string()]
                    },
                    valves_open: self.valves_open + v.index,
                    minutes_left: if idx == 0 {
                        vec![self.minutes_left[idx] - (distances[&(self.current[idx].to_string(), v.name.to_string())] + 1), self.minutes_left[1]]
                    } else {
                        vec![self.minutes_left[0], self.minutes_left[idx] - (distances[&(self.current[idx].to_string(), v.name.to_string())] + 1)]
                    },
                    total_flow: self.total_flow + ((self.minutes_left[idx] - (distances[&(self.current[idx].to_string(), v.name.to_string())] + 1)) * v.flow)
                }
            )
            .collect()
    }

    fn next(&self, valves: &HashMap<String, Valve>, distances: &HashMap<(String, String), i32>) -> Vec<StateWithElephant> {
        let mut result = self.next_index(0, valves, distances);
        if self.minutes_left[0] < self.minutes_left[1] {
            result.append(&mut self.next_index(1, valves, distances));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1() {
        const INPUT: &str = include_str!("../inputs/day16.txt");
        let valves = parse(INPUT.lines());
        let distances = compute_distances(&valves);

        let mut open: Vec<State> = vec![State::new(&valves)];
        let mut max_flow = 0;

        while let Some(current) = open.pop() {
            max_flow = max_flow.max(current.total_flow);
            let mut next = current.next(&valves, &distances);
            open.append(&mut next);
        }

        println!("{}", max_flow);
    }

    #[test]
    fn day16_part2() {
        const INPUT: &str = include_str!("../inputs/day16.txt");
        let valves = parse(INPUT.lines());
        let distances = compute_distances(&valves);

        let mut open: Vec<StateWithElephant> = vec![StateWithElephant::new(&valves)];
        let mut max_flow = 0;
        let mut open2: Vec<StateWithElephant> = vec![];

        while let Some(current) = open.pop() {
            println!("{}", max_flow);
            //println!("({},{}),({},{}),{}", current.current[0], current.current[1], current.minutes_left[0], current.minutes_left[1], current.total_flow);
            max_flow = max_flow.max(current.total_flow);
            let mut next = current.next_index(0, &valves, &distances);
            if next.is_empty() {
                open2.push(current);
            } else {
                open.append(&mut next);
            }
            // open.sort_by(|x, y| x.total_flow.cmp(&y.total_flow))
        }

        while let Some(current) = open2.pop() {
            println!("{} {}", max_flow, open2.len());
            //println!("({},{}),({},{}),{}", current.current[0], current.current[1], current.minutes_left[0], current.minutes_left[1], current.total_flow);
            max_flow = max_flow.max(current.total_flow);
            let mut next = current.next_index(1, &valves, &distances);
            open2.append(&mut next);
            // open.sort_by(|x, y| x.total_flow.cmp(&y.total_flow))
        }

        println!("{}", max_flow);
    }
}
