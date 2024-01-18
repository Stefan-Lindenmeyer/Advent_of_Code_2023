use std::vec;

/*
Task:

The input lists multiple sequences of numbers.

Part One:
Calculate the next number in every sequence by calculating the difference between two adjecent numbers to create a new sequence, repeating this until the sequence consists only of zeros and going back up in the last column.
Then add all new values together for the answer.

Part Two:
Find the previous number in each sequence instead of the next.
Then add all new values together for the answer.

*/

fn main() {}

#[test]
fn test_one() {
    let input = include_str!("test1.txt");
    let parsed = parse(input);
    let res = solve_one(&parsed);
    assert_eq!(res, 114);
}

#[test]
fn test_two() {
    let input = include_str!("test2.txt");
    let parsed = parse(input);
    let res = solve_one(&parsed);
    assert_eq!(res, -18);
}

#[test]
fn test_three() {
    let input = include_str!("test3.txt");
    let parsed = parse(input);
    let res = solve_two(&parsed);
    assert_eq!(res, 5);
}

#[test]
fn part_one() {
    let input = include_str!("input.txt");
    let parsed = parse(&input);
    let res = solve_one(&parsed);
    assert_eq!(res, 1479011877);
}

#[test]
fn part_two() {
    let input = include_str!("input.txt");
    let parsed = parse(&input);
    let res = solve_two(&parsed);
    assert_eq!(res, 973);
}

fn solve_one(data: &Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    for line in data {
        let mut last_column: Vec<i32> = vec![];
        let mut current_row: Vec<i32> = line.clone();
        while !current_row.iter().all(|x| *x == 0) {
            last_column.push(*current_row.last().unwrap());
            let mut next_row = vec![];
            for s in current_row.windows(2) {
                next_row.push(s[1] - s[0])
            }
            current_row = next_row;
        }
        result += last_column.into_iter().sum::<i32>();
    }

    result
}

fn solve_two(data: &Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;

    for line in data {
        let mut first_column: Vec<i32> = vec![];
        let mut current_row: Vec<i32> = line.clone();
        while !current_row.iter().all(|x| *x == 0) {
            first_column.push(*current_row.first().unwrap());
            let mut next_row = vec![];
            for s in current_row.windows(2) {
                next_row.push(s[1] - s[0])
            }
            current_row = next_row;
        }
        result += first_column.into_iter().rev().reduce(|acc, e| e - acc).unwrap();
    }

    result
}

fn parse(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| i32::from_str_radix(x, 10).unwrap())
                .collect()
        })
        .collect()
}
