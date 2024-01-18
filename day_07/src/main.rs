use std::{cmp::Ordering, collections::HashMap};

/*
Task:

The input lists poker hands with their bid amounts.
Each Hand wins its bid amount multiplied by its rank compared to the other Hands.

Part One:
Calculate the total winnings.

Part Two:
A J inside a Hand can take any value now, but is the weakest individual card.
Calculate the total winnings.

*/

#[derive(Debug)]
struct Hand {
    labels: String,
    hex_labels: u32,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hex_labels == other.hex_labels
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hex_labels.cmp(&other.hex_labels)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

fn main() {}

#[test]
fn test_one() {
    let input = include_str!("test.txt");
    let parsed = parse(&input, false);
    let res = solve(&parsed, false);
    assert_eq!(res, 6440);
}

// #[test]
// fn test_two() {
//     let input = include_str!("test_01.txt");
//     // All Hands must have this label
//     // hex_labels: labels_to_int(&"QQQJA".to_string(), is_part_two);
//     let parsed = parse(&input, true);
//     // println!("{:?}", parsed);
//     // parsed.sort();
//     // println!("{:?}", parsed);
//     let res = solve(&parsed, true);
//     assert_eq!(res, 6440);
// }

#[test]
fn part_one() {
    let input = include_str!("input.txt");
    let parsed = parse(&input, false);
    let res = solve(&parsed, false);
    assert_eq!(res, 247823654);
}

#[test]
fn part_two() {
    let input = include_str!("input.txt");
    let parsed = parse(&input, true);
    let res = solve(&parsed, true);
    assert_eq!(res, 245461700);
}

fn solve(hands: &Vec<Hand>, is_part_two: bool) -> u32 {
    let mut hand_buckets: HashMap<HandType, Vec<&Hand>> = HashMap::new();
    hand_buckets.insert(HandType::Five, vec![]);
    hand_buckets.insert(HandType::Four, vec![]);
    hand_buckets.insert(HandType::FullHouse, vec![]);
    hand_buckets.insert(HandType::Three, vec![]);
    hand_buckets.insert(HandType::TwoPair, vec![]);
    hand_buckets.insert(HandType::OnePair, vec![]);
    hand_buckets.insert(HandType::HighCard, vec![]);

    for hand in hands {
        let hand_type = if is_part_two {
            determine_hand_type_two(&hand.labels)
        } else {
            determine_hand_type(&hand.labels)
        };
        let bucket = hand_buckets.get_mut(&hand_type).unwrap();
        bucket.push(hand);
    }

    let mut result = 0;
    let mut rank = 1;

    //println!("{}, {}, {}, {}", el.labels, el.bid, rank, el.bid * rank);
    hand_buckets.get_mut(&HandType::HighCard).unwrap().sort();
    for hand in &hand_buckets[&HandType::HighCard][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::OnePair).unwrap().sort();
    for hand in &hand_buckets[&HandType::OnePair][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::TwoPair).unwrap().sort();
    for hand in &hand_buckets[&HandType::TwoPair][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::Three).unwrap().sort();
    for hand in &hand_buckets[&HandType::Three][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::FullHouse).unwrap().sort();
    for hand in &hand_buckets[&HandType::FullHouse][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::Four).unwrap().sort();
    for hand in &hand_buckets[&HandType::Four][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    hand_buckets.get_mut(&HandType::Five).unwrap().sort();
    for hand in &hand_buckets[&HandType::Five][..] {
        result += hand.bid * rank;
        rank += 1;
    }

    //println!("{}", result);
    result
}

fn determine_hand_type_two(labels: &String) -> HandType {
    let mut labels_ordered: Vec<char> = labels.chars().collect();
    labels_ordered.sort_unstable();

    let mut counts: Vec<u32> = vec![];
    let mut cur = labels_ordered[0];
    let mut count = 0;
    let mut js = 0;
    for l in labels_ordered {
        if l == 'J' {
            js += 1;
        }
        if l == cur {
            count += 1
        } else {
            counts.push(count);
            count = 1;
            cur = l;
        }
    }
    counts.push(count);

    //println!("{:?}, {:?}", labels_ordered, counts);

    match counts[..] {
        [5] => HandType::Five,                                   // Five of a kind
        [4, _] | [_, 4] if js == 1 || js == 4 => HandType::Five, //4+J || JJJJ+1
        [4, _] | [_, 4] => HandType::Four,                       // Four of a kind
        [3, 2] | [2, 3] if js == 2 || js == 3 => HandType::Five, //3+JJ || 2+JJJ
        [3, 2] | [2, 3] => HandType::FullHouse,                  // Full House
        [3, ..] | [_, 3, _] | [.., 3] if js == 1 || js == 3 => HandType::Four, //3+J+1 || JJJ+1+1
        [3, ..] | [_, 3, _] | [.., 3] => HandType::Three,        // Three of a kind
        [_, _, _] if js == 2 => HandType::Four,                  //2+JJ+1
        [_, _, _] if js == 1 => HandType::FullHouse,             //2+2+J
        [_, _, _] => HandType::TwoPair,                          // Two Pair
        [_, _, _, _] if js == 2 || js == 1 => HandType::Three,   //JJ+1+1+1 || 2+J+1+1
        [_, _, _, _] => HandType::OnePair,                       // One Pair
        _ if js == 1 => HandType::OnePair,                       //J+1+1+1
        _ => HandType::HighCard,                                 // High Card
    }
}

fn determine_hand_type(labels: &String) -> HandType {
    let mut labels_ordered: Vec<char> = labels.chars().collect();
    labels_ordered.sort_unstable();

    let mut counts: Vec<u32> = vec![];
    let mut cur = labels_ordered[0];
    let mut count = 0;
    for l in labels_ordered {
        if l == cur {
            count += 1
        } else {
            counts.push(count);
            count = 1;
            cur = l;
        }
    }
    counts.push(count);

    //println!("{:?}, {:?}", labels_ordered, counts);

    match counts[..] {
        [5] => HandType::Five,                            // Five of a kind
        [4, _] | [_, 4] => HandType::Four,                // Four of a kind
        [3, 2] | [2, 3] => HandType::FullHouse,           // Full House
        [3, ..] | [_, 3, _] | [.., 3] => HandType::Three, // Three of a kind
        [_, _, _] => HandType::TwoPair,                   // Two Pair
        [_, _, _, _] => HandType::OnePair,                // One Pair
        _ => HandType::HighCard,                          // High Card
    }
}

fn labels_to_int(labels: &String, part_two: bool) -> u32 {
    let hex_labels = if part_two {
        labels
            .replace('J', "1")
            .replace('Q', "B")
            .replace('K', "C")
            .replace('A', "D")
            .replace('T', "A")
    } else {
        labels
            .replace('J', "B")
            .replace('Q', "C")
            .replace('K', "D")
            .replace('A', "E")
            .replace('T', "A")
    };

    //println!("{}, {}", labels, hex_labels);

    u32::from_str_radix(&hex_labels, 16).expect("Not a hex string!")
}

fn parse(data: &str, is_part_two: bool) -> Vec<Hand> {
    data.lines()
        .map(|line| {
            let (labels, bid) = line.trim().split_once(' ').unwrap();
            let bid: u32 = bid.parse().unwrap();
            Hand {
                labels: labels.to_string(),
                hex_labels: labels_to_int(&labels.to_string(), is_part_two),
                bid,
            }
        })
        .collect()
}
