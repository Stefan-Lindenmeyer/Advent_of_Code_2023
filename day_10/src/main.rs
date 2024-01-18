use std::vec;

/*
Task:

The input lists a map of pipes.

Part One:
Given the start point S, calculate the distance to the point farthest away in the pipe loop.

Part Two:
Calculate the total inside area of the pipe loop.

*/


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn is_to_left_of(&self, c: &Coordinate) -> bool {
        self.x < c.x
    }
    fn is_to_right_of(&self, c: &Coordinate) -> bool {
        self.x > c.x
    }
    fn is_above(&self, c: &Coordinate) -> bool {
        self.y < c.y
    }
    fn is_below(&self, c: &Coordinate) -> bool {
        self.y > c.y
    }
}

fn main() {}

#[test]
fn test_one() {
    let test = include_str!("test1.txt");
    let res = solve_one(&test);
    assert_eq!(res, (4, 2));
}

#[test]
fn test_two() {
    let test = include_str!("test2.txt");
    let res = solve_one(&test);
    assert_eq!(res, (8, 0));
}

#[test]
fn part_one() {
    let test = include_str!("input.txt");
    let res = solve_one(&test);
    assert_eq!(res, (6875, 471));
}

fn go_up(next: &mut Coordinate, current: &mut Coordinate) {
    (current.x, current.y, next.x, next.y) = (next.x, next.y, next.x, next.y - 1);
}

fn go_down(next: &mut Coordinate, current: &mut Coordinate) {
    (current.x, current.y, next.x, next.y) = (next.x, next.y, next.x, next.y + 1);
}

fn go_left(next: &mut Coordinate, current: &mut Coordinate) {
    (current.x, current.y, next.x, next.y) = (next.x, next.y, next.x - 1, next.y);
}

fn go_right(next: &mut Coordinate, current: &mut Coordinate) {
    (current.x, current.y, next.x, next.y) = (next.x, next.y, next.x + 1, next.y);
}

fn solve_one(data: &str) -> (i32, i32) {
    let mut result: i32 = 0;

    let rows: Vec<&str> = data.lines().collect();
    let width = rows[0].len();
    let (y, start_row) = rows
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains('S'))
        .unwrap();
    let start = Coordinate {
        x: start_row.find('S').unwrap(),
        y,
    };

    let mut current = start.clone();
    let mut last = start.clone();

    // Look in each direction to find next(current) tile
    // I assume S in surrounded by tiles and not on the edge
    if start.x > 0 {
        // left
        match rows[start.y].chars().nth(start.x - 1).unwrap() {
            '-' | 'L' | 'F' => {
                current.x = start.x - 1;
                current.y = start.y;
            }
            _ => (),
        }
    }
    if start.x < width {
        // right
        match rows[start.y].chars().nth(start.x + 1).unwrap() {
            '-' | 'J' | '7' => {
                current.x = start.x + 1;
                current.y = start.y;
            }
            _ => (),
        }
    }
    if start.y > 0 {
        // top
        match rows[start.y - 1].chars().nth(start.x).unwrap() {
            '|' | 'F' | '7' => {
                current.x = start.x;
                current.y = start.y - 1;
            }
            _ => (),
        }
    }
    if start.y < rows.len() {
        // bottom
        match rows[start.y + 1].chars().nth(start.x).unwrap() {
            '|' | 'J' | 'L' => {
                current.x = start.x;
                current.y = start.y + 1;
            }
            _ => (),
        }
    }

    let mut visited_positions = vec![current];
    
    // Calculate where to move next based on where we come from
    loop {
        // input loop is closed, no edge cases exist if input is correct
        let current_tile = rows[current.y].chars().nth(current.x).unwrap();
        match current_tile {
            '|' => {
                if current.is_above(&last) {
                    go_up(&mut current, &mut last);
                } else {
                    go_down(&mut current, &mut last);
                }
            }
            '-' => {
                if current.is_to_left_of(&last) {
                    go_left(&mut current, &mut last);
                } else {
                    go_right(&mut current, &mut last);
                }
            }
            'L' => {
                if current.is_to_left_of(&last) {
                    go_up(&mut current, &mut last);
                } else {
                    go_right(&mut current, &mut last);
                }
            }
            'J' => {
                if current.is_to_right_of(&last) {
                    go_up(&mut current, &mut last);
                } else {
                    go_left(&mut current, &mut last);
                }
            }
            '7' => {
                if current.is_to_right_of(&last) {
                    go_down(&mut current, &mut last);
                } else {
                    go_left(&mut current, &mut last);
                }
            }
            'F' => {
                if current.is_to_left_of(&last) {
                    go_down(&mut current, &mut last);
                } else {
                    go_right(&mut current, &mut last);
                }
            }
            '.' => panic!("Invalid value! {current:?}"),
            'S' => break,
            x => panic!("Invalid value! {x}"),
        }
        visited_positions.push(current);
        result += 1;
    }

    // I had the scanline idea too but could not get it to work in the time I had, Part two borrowed (and modified) from
    // https://topaz.github.io/paste/#XQAAAQANCgAAAAAAAAA6nMjJFC0J/CJZCRCcqDK7ekZNlLB87tjzyjORRYfebrfqgDU9ntKWvcOTqT+/5JHb7XFT/88GI+gTrAX6okz/x55+K4DZ7O9G4Tyv+gr5c1PRgOjZ23WJvzMl2CrnE3di7klPTpDxNGQVKcZ2OiGHWn92TNjootVJ1w8PY2pqPW1o/K7QJ3aWTk1CSPNPua1XqCdYKDElcF+KTvTLkFG+XIdKMcDm/OhiYOnGJpkouFcczUt4jg/2ATZFkZo4cfZIM3pZh1hXqX93OiKl/aNmnSyA87ipt2cIIWLEilA5OASlrtxfuIBxPIDfU+c+sPfZvCelBHazRSH5FKdf6sRfWZMVoFFFP1p5jRzC/mc5soTiShCwlVSIuVPWU3rmN82DR6k+n5FXBk/jy4IR8QoWzB7MJXE2lVnwbj94fUZWwObOZIF0b15OQagVwhtsSYMZ0m3wrneNj3B3NQGGTT1dXbDR2ziNtkv1rlzPS5ojTCNKDD8taee66tP9+m67xHCBc52/rEE6m7qzj7A03PJD0ugHlhRMdFCjudq+qNQ5KX1UVJ9PFOElAYDZYbKggam2BVw/dyHd1JdD3Sy7M7M8Umi6xENVIgHT0Ro3XVHcfWg+j/QbFSQzn0DqQ5m4yR142qVgsAvo4GF1/Asv3FnLmrCggcXoP/D/tgcauRAYyLp2bz5H9Daf43bKurzsV2YMOIna3e8XGJ8xXlJliOxq1OfaDTFMgcJc/zpG1/ENGqI9ubYCxaSVJYO3hy7mWkePUxOGOdLmU1hz15my+IA4yV2SWrHAz0D86rfgYsLwBhNHegLDPFhgjj0DU06yE8WiTM3USvf8Yvt7A/cQwTEKbQj64xZ4bqtnueNwFjMCvWhBE8FD7XWYRKkK1ucjwT8ZIJv15EoBjxCbQ1Gv9TSMRUvkEQFpo5ejvrNAPzyjJViphrQpVYPub1ia58ho1w1aheaf6+BNxBAAC3BNS84biEHxvNeyG+RHFlthirfCsqKpAbPydgCf9oRvmsfNEK0FTcP3s0DxEXCuxaB6G2jj6nu0mNtP0xLeyDm+OLUPIUq8fvgp70xcfhQuvfgB/9D4VMsDG4dKMFD8+KmOBXhafTbI4Yi6C04T4rF4l4wXZE55SizdKFXP1HS7qqFw9IPoptFLnc18DcWPshLN1pZppBy6HvTQJSra1tIAAcGI3mh2UWnn1sKiSMsPfc0Frrf/5UGh4A==
    // https://www.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions/kctmc3c/

    // Part two start

    let mut inside_tiles = 0;
    for (y, n) in rows.iter().enumerate() {
        let mut inside = false;
        for x in 0..width {
            // If we cross the path, we go from outside to inside and vice versa
            if visited_positions.contains(&&Coordinate { x, y }) {
                // We only cross the path if we cross the "bottom half" of a pipe:
                // That is any pipe that connects to the south
                match n.chars().nth(x).unwrap() {
                    '|' | '7' | 'F' => inside = !inside,
                    _ => (),
                }
            } else if inside {
                inside_tiles += 1;
            }
        }
    }
    // Part two end

    (result / 2 + result % 2, inside_tiles)
}
