/*
Task:

The input lists a map of space and galaxies in that space.


Part One:
Each row or column in which no galaxy is found is actually two rows or columns.
Calculate the sum of shortest paths between every pair of galaxies.

Part Two:
Same as part one but the multiplier for each empty row is now 1_000_000 instead of two.

*/

fn main() {}

#[test]
fn test_one() {
    let test = include_str!("test1.txt");
    let res = solve(&test, 2);
    assert_eq!(res, 374);
}

#[test]
fn test_two() {
    let test = include_str!("test1.txt");
    let res = solve(&test, 10);
    assert_eq!(res, 1030);
}

#[test]
fn test_two_5() {
    let test = include_str!("test2.txt");
    let res = solve(&test, 10);
    assert_eq!(res, 22);
}

#[test]
fn test_three() {
    let test = include_str!("test1.txt");
    let res = solve(&test, 100);
    assert_eq!(res, 8410);
}

#[test]
fn part_one() {
    let input = include_str!("input.txt");
    let res = solve(&input, 2);
    assert_eq!(res, 9623138);
}

#[test]
fn part_two() {
    let input = include_str!("input.txt");
    let res = solve(&input, 1_000_000);
    assert_eq!(res, 726820169514);
}

fn solve(data: &str, mult: u64) -> u64 {
    // Get coordinates of all galaxies and expand coordinates in y direction
    let mut empty_row_counter = 0;
    let mut test: Vec<(u64, u64)> = data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let galaxies: Vec<(u64, u64)> = line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as u64, y as u64 + empty_row_counter * (mult - 1)))
                    } else {
                        None
                    }
                })
                .collect();
            if galaxies.is_empty() {
                empty_row_counter += 1;
            }
            galaxies
        })
        .flatten()
        .collect();

    // Expand coordinates in x-direction
    //println!("{test:?}");
    test.sort_by(|a, b| a.0.cmp(&b.0));
    let mut marked_x: u64 = 0; // Marks the border on which x's we did not check yet to find a galaxy
    let mut empty_column_counter = 0;
    test.iter_mut().for_each(|c| {
        empty_column_counter += c.0.checked_sub(marked_x).unwrap_or(0);
        marked_x = c.0 + 1;
        c.0 += empty_column_counter * (mult - 1);
    });
    //println!("{test:?}");

    // Compute pair distances
    let t: Vec<u64> = test
        .iter()
        .enumerate()
        .map(|(i, c)| {
            test.as_slice()[i..]
                .iter()
                .fold(0, |acc, x| acc + c.0.abs_diff(x.0) + c.1.abs_diff(x.1))
        })
        .collect();
    // println!("{t:?}");
    t.into_iter().sum()
}
