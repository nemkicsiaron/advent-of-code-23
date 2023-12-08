use std::collections::HashMap;
use std::convert::TryInto;

mod one;

fn rank(card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _    => card.to_digit(10).unwrap().try_into().unwrap()
    }
}

fn compare_hands(lhs: &str, rhs: &str) -> std::cmp::Ordering {
    let lpow = hand_power(lhs);
    let rpow = hand_power(rhs);
    let mut res: std::cmp::Ordering = std::cmp::Ordering::Equal;
    if lpow > rpow {
        res = std::cmp::Ordering::Greater;
    } else if lpow < rpow {
        res = std::cmp::Ordering::Less;
    }

    let mut i = 0;
    while res == std::cmp::Ordering::Equal && i < lhs.len() && i < rhs.len() {
        let l = lhs.chars().nth(i).unwrap();
        let r = rhs.chars().nth(i).unwrap();
        //println!("{0} has rank {1} while {2} has rank {3}", l, rank(l), r, rank(r));
        if rank(l) > rank(r) {
            res = std::cmp::Ordering::Greater;
        } else if rank(l) < rank(r) {
            res = std::cmp::Ordering::Less;
        }
        i += 1;
    }
    /*match res {
        std::cmp::Ordering::Greater => {println!("{0} is greater than {1} because {2} is greater or it's order was better {3}", lhs, rhs, lpow, rpow);}
        std::cmp::Ordering::Less => {println!("{0} is less than {1} because {2} is less or it's order was worse {3}", lhs, rhs, lpow, rpow);}
        _ =>{println!("{0} seems samey to {1} because {2} is equal to {3}", lhs, rhs, lpow, rpow);}
    }*/
    return res;
}

fn hand_power(hand: &str) -> i32 {
    let mut cards: HashMap<char, i32> = HashMap::new();
    let mut pow = 1;
    //assert_eq!(cards.get(&'A'), None);

    for char in hand.chars() {
        cards.entry(char).and_modify(|x| *x += 1).or_insert(1);
    }

    let jokers = cards.remove(&'J').unwrap_or(0);
    let bestcard = cards.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap_or(&'A');
    //println!("{}", bestcard);
    cards.entry(*bestcard).and_modify(|x| *x += jokers);

    let cards_len = cards.len();

    if cards_len == 1 {
        pow = 7;
    } else if cards_len == 2 {
        let v: Vec<&i32> = cards.values().collect();
        if v.contains(&&4) {
            pow = 6
        } else {
            pow = 5;
        }
    } else if cards_len == 3 {
        let v: Vec<&i32> = cards.values().collect();
        if v.contains(&&3) {
            pow = 4
        } else {
            pow = 3;
        }
    } else if cards_len == 4 {
        pow = 2
    }

    return pow;
}

fn part_two(arr: Vec<&str>) -> i32 {
    let mut total: i32 = 0;
    let mut hands = Vec::new();
    let mut bets: HashMap<&str, i32> = HashMap::new();

    for line in arr {
        let (hand, bet) = line.split_once(" ").unwrap();
        hands.push(hand);
        bets.insert(hand, bet.parse().unwrap());
    }

    hands.sort_by(|a, b| compare_hands(a,b));

    for (i, hand) in hands.iter().enumerate() {
        let addage: i32 = (i as i32) + 1;
        total += addage * bets.get(hand).unwrap();
    }

    //println!("{:?}", hands);
    //println!("{:?}", bets);

    return total;
}

pub fn main() {
    let input = include_str!("../input.txt");
    let arr: Vec<&str> = input.split("\r\n").collect();

    //println!("{:?}", arr);
    println!("{:?}", part_two(arr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("../example.txt");
        let arr: Vec<&str> = input.split("\r\n").collect();
        let result = one::part_one(arr);
        assert_eq!(result, 6440);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("../example.txt");
        let arr: Vec<&str> = input.split("\r\n").collect();
        let result = part_two(arr);
        assert_eq!(result, 5905);
    }
}