use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::day7::HandType::{
    FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair,
};
use crate::util::get_non_empty_lines;

#[derive(Debug, Eq, PartialEq, Hash)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

lazy_static! {
    static ref CARD_ORDER: [char; 13] =
        ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    static ref CARD_POWER: HashMap<char, u32> = {
        CARD_ORDER
            .iter()
            .rev()
            .zip(1..)
            .map(|(&s, i)| (s, i))
            .collect()
    };
    static ref PART2_CARD_ORDER: [char; 13] =
        ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    static ref PART2_CARD_POWER: HashMap<char, u32> = {
        PART2_CARD_ORDER
            .iter()
            .rev()
            .zip(1..)
            .map(|(&s, i)| (s, i))
            .collect()
    };
    static ref HAND_POWER: HashMap<HandType, u32> = HashMap::from([
        (HighCard, 1),
        (OnePair, 2),
        (TwoPair, 3),
        (ThreeOfKind, 4),
        (FullHouse, 5),
        (FourOfKind, 6),
        (FiveOfKind, 7)
    ]);
}

#[derive(Debug)]
struct RawHand<'a> {
    cards: &'a str,
    bid: u64,
}
#[derive(Debug)]
struct HandTypeAndPower {
    #[allow(dead_code)]
    typ: HandType,
    power: u32,
}
#[derive(Debug)]
struct HandAndType<'a> {
    hand: RawHand<'a>,
    hand_type: HandTypeAndPower,
}

fn parse_hands(data: &str) -> Vec<RawHand> {
    let re = Regex::new(r"(.{5}) (\d+)").unwrap();
    let lines = get_non_empty_lines(data);

    lines
        .map(|line| {
            let (_, [cards, bid]) = re
                .captures(line.line)
                .unwrap_or_else(|| panic!("{line}"))
                .extract();

            RawHand {
                cards,
                bid: bid.parse().unwrap_or_else(|e| panic!("{e}: {line}")),
            }
        })
        .collect()
}

fn get_type(cards: &str) -> HandTypeAndPower {
    let mut m = HashMap::new();
    cards.chars().for_each(|c| {
        m.entry(c).and_modify(|count| *count += 1).or_insert(1);
    });

    let mut counts = m.into_values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    let highest = counts[0];

    let typ = match highest {
        5 => FiveOfKind,
        4 => FourOfKind,
        3 => {
            let second = counts[1];
            if second == 2 {
                FullHouse
            } else {
                ThreeOfKind
            }
        }
        2 => {
            let second = counts[1];
            if second == 2 {
                TwoPair
            } else {
                OnePair
            }
        }
        _ => HighCard,
    };

    let power = *HAND_POWER.get(&typ).unwrap();

    HandTypeAndPower { typ, power }
}

fn doit_impl<F>(data: &str, enrich: F, card_power_map: &HashMap<char, u32>) -> u64
where
    F: FnMut(RawHand) -> HandAndType,
{
    let card_to_power = |card| *card_power_map.get(&card).unwrap();

    let hands = parse_hands(data);
    let mut hands: Vec<_> = hands.into_iter().map(enrich).collect();

    hands.sort_by(|a, b| {
        let ord = a.hand_type.power.cmp(&b.hand_type.power);
        if ord != Ordering::Equal {
            ord
        } else {
            a.hand
                .cards
                .chars()
                .map(card_to_power)
                .cmp(b.hand.cards.chars().map(card_to_power))
        }
    });

    hands
        .iter()
        .zip(1..)
        .map(|(h, rank)| h.hand.bid * rank)
        .sum()
}

fn doit(data: &str) -> u64 {
    fn enrich(hand: RawHand) -> HandAndType {
        let hand_type = get_type(hand.cards);
        HandAndType { hand, hand_type }
    }

    doit_impl(data, enrich, &CARD_POWER)
}

fn get_possible_hands(non_jokers: &HashSet<char>, hand: &str) -> Vec<String> {
    fn rec(non_jokers: &HashSet<char>, acc: String, rem: &[char]) -> Vec<String> {
        match rem.first() {
            None => {
                vec![acc]
            }
            Some(&'J') => non_jokers
                .iter()
                .flat_map(|&char| {
                    let mut new_rem = vec![char];
                    new_rem.extend_from_slice(&rem[1..]);
                    rec(non_jokers, acc.clone(), &new_rem)
                })
                .collect(),
            Some(&curr) => rec(non_jokers, format!("{acc}{curr}"), &rem[1..]),
        }
    }

    rec(non_jokers, String::new(), &hand.chars().collect::<Vec<_>>())
}

fn doit2(data: &str) -> u64 {
    fn replace_jokers(hand: &RawHand) -> Vec<HandTypeAndPower> {
        let mut cards_in_hand: HashSet<_> = hand.cards.chars().collect();
        if !cards_in_hand.contains(&'J') {
            vec![get_type(hand.cards)]
        } else {
            cards_in_hand.remove(&'J');

            // 5 Js, eh
            if cards_in_hand.is_empty() {
                vec![get_type(hand.cards)]
            } else {
                get_possible_hands(&cards_in_hand, hand.cards)
                    .iter()
                    .map(|s| get_type(s))
                    .collect()
            }
        }
    }

    fn enrich(hand: RawHand) -> HandAndType {
        let all_hands = replace_jokers(&hand);
        let best = all_hands
            .into_iter()
            .reduce(|a, b| if a.power > b.power { a } else { b })
            .unwrap_or_else(|| panic!("{:?}", hand));
        HandAndType {
            hand,
            hand_type: best,
        }
    }

    doit_impl(data, enrich, &PART2_CARD_POWER)
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day7/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 6440);

        let answer = doit2(data);
        assert_eq!(answer, 5905)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day7/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 246163188);

        let answer = doit2(data);
        assert_eq!(answer, 245794069)
    }
}
