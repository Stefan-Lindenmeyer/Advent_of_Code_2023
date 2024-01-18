/*  Task:

The input lists race times (ms) and respective distances (mm).
A button can be pressed to increase the speed of a boat by 1 mm/ms for each ms pressed.
After release the boat will travel at that speed for the race duration.

Part One:
For each race time find out how many ways to hold the button there are to beat the given distance.
Multiply the results together to get the answer.

Part Two:
Same as Part One but only one race with all time and distance values concatenated.

*/

fn main() {}

#[test]
fn part_one() {
    let input: [(f32, f32); 4] = [
        (41.0, 214.0),
        (96.0, 1789.0),
        (88.0, 1127.0),
        (94.0, 1055.0),
    ];
    let res = solve_one(input);
    assert_eq!(res, 4811940);
}

#[test]
fn part_two() {
    let input: [(f64, f64); 1] = [(41968894.0, 214178911271055.0)];
    let res = solve_two(input);
    assert_eq!(res, 30077773);
}

fn solve_one(v: [(f32, f32); 4]) -> u32 {
    // Rectangle optimization
    // total area: tx-x^2
    // t is time
    // d is distance
    // push parabola down by d: -x^2+tx-d
    // calculate intersection with x-axis
    // get all int values between intersections

    let mut result = 1;

    for td in v {
        let a = -1.0;
        let time = td.0;
        let distance = -td.1;

        // -b +- sqrt(b^2 - 4ac)  / 2a
        let left_intersection = (-time + (time.powf(2.0) - 4.0 * a * distance).sqrt()) / (2.0 * a);
        let right_intersection = (-time - (time.powf(2.0) - 4.0 * a * distance).sqrt()) / (2.0 * a);

        let mut mv = left_intersection.min(right_intersection);
        if mv.fract() <= f32::EPSILON {
            mv = mv + 0.5;
        }

        result *= right_intersection.max(left_intersection).ceil() as u32 - mv.ceil() as u32;
    }

    result
}

fn solve_two(v: [(f64, f64); 1]) -> u32 {
    // same code as part one except for the input type

    let mut result = 1;

    for td in v {
        let a = -1.0;
        let time = td.0;
        let distance = -td.1;

        // -b +- sqrt(b^2 - 4ac)  / 2a
        let left_intersection = (-time + (time.powf(2.0) - 4.0 * a * distance).sqrt()) / (2.0 * a);
        let right_intersection = (-time - (time.powf(2.0) - 4.0 * a * distance).sqrt()) / (2.0 * a);

        let mut mv = left_intersection.min(right_intersection);
        if mv.fract() <= f64::EPSILON {
            mv += 0.5;
        }

        result *= right_intersection.max(left_intersection).ceil() as u32 - mv.ceil() as u32;
    }

    result
}
