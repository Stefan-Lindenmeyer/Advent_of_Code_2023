use std::fs::read_to_string;

#[derive(Debug)]
struct BinMap<'a> {
    root: Option<&'a Node<'a>>,
    nodes: Vec<Node<'a>>,
    directions: Vec<bool>,
}

impl<'a> BinMap<'a> {
    // fn insert_old(&mut self, value: String, left: String, right: String) {
    //     let mut node = Node {
    //         value,
    //         left: None,
    //         right: None,
    //     };

    //     let mut node_ref = &mut node;

    //     let nodes = &mut self.nodes;

    //     match self.find(&value, nodes) {
    //         Some(n) => {
    //             if n.left.is_none() {
    //                 node_ref = n;
    //             } else {
    //                 return;
    //             }
    //         }
    //         None => (),
    //     }

    //     match self.find(&left, nodes) {
    //         Some(n) => node_ref.left = Some(n),
    //         None => self.nodes.push(Node {
    //             value: left,
    //             left: None,
    //             right: None,
    //         }),
    //     }

    //     match self.find(&right, nodes) {
    //         Some(n) => node.right = Some(n),
    //         None => self.nodes.push(Node {
    //             value: right,
    //             left: None,
    //             right: None,
    //         }),
    //     }

    //     if node.value == "AAA" && self.root.is_some() {
    //         self.root = Some(node_ref)
    //     }
    //     nodes.push(node);
    // }

    // fn find(&mut self, value: &String, nodes: &mut Vec<Node> ) -> Option<&'a mut Node> {
    //     for n in nodes.iter_mut() {
    //         if n.value == *value {
    //             return Some(n);
    //         }
    //     }
    //     None
    // }

    fn insert(&mut self, value: String, left: String, right: String) {
        if self.contains(&value) {return;}

        self.nodes.push(Node { value: value, left: self.get(&left), right: None });
    }

    fn contains(&self, value: &String) -> bool {
        for n in self.nodes.iter() {
            if n.value == *value {
                return true;
            }
        }
        false
    }

    fn get<'b>(&'b self, value: &String) -> Option<&'b Node<'a>>{
        for n in self.nodes.iter() {
            if n.value == *value {
                return Some(n);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Node<'a> {
    value: String,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

fn main() {
    let test_one = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let test_two = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let input_one = parse(
        "C:/Users/stefa/Documents/GitHub/Advent_of_Code_2023/day_8/src/map.txt",
        false,
    );
    let input_two = parse(
        "C:/Users/stefa/Documents/GitHub/Advent_of_Code_2023/day_8/src/map.txt",
        true,
    );

    let part_two = true;

    let mut v: BinMap = BinMap {
        root: None,
        nodes: vec![],
        directions: vec![],
    };

    for line in test_one.lines() {
        process_line(line, &mut v)
    }

    print!("{:?}", v);

    // Start 14:45
    //
    part_one(&v); // 18:47

    //
    //part_one(&input_two, true); // 19:44
}

fn part_one(v: &BinMap) {
    //let mut current_node = v.root.unwrap();
    let mut count = 0;
    // while current_node.value != "ZZZ" {
    //     if v.directions[count % v.directions.len()] {
    //         current_node = &mut current_node.left.unwrap();
    //     } else {
    //         current_node = &mut current_node.right.unwrap();
    //     }
    //     count += 1;
    // }

    // //let mut result = 0;

    println!("{}", count);
}

fn process_line(line: &str, v: &mut BinMap) {
    if v.directions.is_empty() {
        let mut dirs: Vec<bool> = vec![];
        let directions = line.trim();
        for c in directions.chars() {
            dirs.push(if c == 'L' { true } else { false });
        }
        v.directions = dirs;
        return;
    }

    if line.trim().is_empty() {
        return;
    }

    let n: Vec<&str> = line.trim().split(" = ").collect();
    //println!("{:?}", n);
    let lr: Vec<&str> = n[1]
        .trim_end_matches(')')
        .trim_start_matches('(')
        .split(", ")
        .collect();

    v.insert(n[0].to_string(), lr[0].to_string(), lr[1].to_string());
}

fn parse(filename: &str, part_two: bool) -> BinMap {
    let mut v: BinMap = BinMap {
        root: None,
        nodes: vec![],
        directions: vec![],
    };

    for line in read_to_string(filename).unwrap().lines() {
        process_line(line, &mut v)
    }

    v
}
