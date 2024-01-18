use std::{collections::HashMap, vec};

/*
Task:

The input lists a "binary network" and instructions (Go left or right) how to traverse it.
The instructions can be repeated if needed.
AAA is the start and ZZZ is the end.

Part One:
Calculate the how many steps are required to traverse the tree.

Part Two:
Now every node that ends in A is a start and every node that ends with Z is an end.
Calculate the how many steps are required to traverse the tree for every start node to reach an end node at the same time.

*/


#[derive(Debug)]
struct Data {
    hmap: HashMap<String, (String, String)>,
    directions: Vec<bool>,
    start_nodes: Vec<String>,
}

fn main() {}

#[test]
fn test_one() {
    let input = include_str!("test_01.txt");
    let parsed = parse(&input);
    let res = solve_one(&parsed);
    assert_eq!(res, 2);
}

#[test]
fn part_one() {
    let input = include_str!("input.txt");
    let parsed = parse(&input);
    let res = solve_one(&parsed);
    assert_eq!(res, 17873);
}

#[test]
fn part_two() {
    let input = include_str!("input.txt");
    let parsed = parse(&input);
    let res = solve_two(&parsed);
    assert_eq!(res, 15746133679061);
}

fn solve_one(v: &Data) -> u32 {
    let mut count = 0;

    let mut current_dir = &v.hmap["AAA"];
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        //println!("{}, {}", v.directions[count % v.directions.len()], current_node);
        if v.directions[count % v.directions.len()] {
            current_node = current_dir.0.as_str();
            current_dir = &v.hmap[&current_dir.0];
        } else {
            current_node = current_dir.1.as_str();
            current_dir = &v.hmap[&current_dir.1];
        }
        count += 1;
    }

    //println!("{}", count);
    count as u32
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn solve_two(input_data: &Data) -> u64 {
    let mut counts: Vec<u64> = vec![];

    for node in input_data.start_nodes.iter() {
        let mut count = 0;
        let mut current_dir = &input_data.hmap[node];
        let mut current_node: &String = &node;

        //print!("{} ", node);

        while !current_node.ends_with('Z') {
            //println!("{}, {}", v.directions[count % v.directions.len()], current_node);
            if input_data.directions[count % input_data.directions.len()] {
                current_node = &current_dir.0;
                current_dir = &input_data.hmap[&current_dir.0];
            } else {
                current_node = &current_dir.1;
                current_dir = &input_data.hmap[&current_dir.1];
            }
            count += 1;
        }
        //println!("{}", count);
        counts.push(count as u64);
    }
    let mut result: u64 = counts[0];

    for c in counts {
        result = (c * result) / (gcd(c, result));
    }

    //println!("{}", result);
    result
}

fn process_line(line: &str, input_data: &mut Data) {
    if input_data.directions.is_empty() {
        input_data.directions = line
            .trim()
            .chars()
            .map(|c| if c == 'L' { true } else { false })
            .collect();
        return;
    }

    if line.trim().is_empty() {
        return;
    }

    let (node, directions) = line.trim().split_once(" = ").unwrap();
    let (left, right) = directions
        .trim_end_matches(')')
        .trim_start_matches('(')
        .split_once(", ")
        .unwrap();
    if node.ends_with('A') {
        input_data.start_nodes.push(node.to_string());
    }
    input_data
        .hmap
        .insert(node.to_string(), (left.to_string(), right.to_string()));
}

fn parse(data: &str) -> Data {
    let mut input_data = Data {
        hmap: HashMap::new(),
        directions: vec![],
        start_nodes: vec![],
    };

    data.lines()
        .for_each(|line| process_line(line, &mut input_data));

    input_data
}
