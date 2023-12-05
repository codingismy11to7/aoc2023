use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::util::{get_non_empty_lines, DataLine};

struct Card {
    id: u64,
    winning_nums: Vec<u64>,
    nums_you_have: Vec<u64>,
}

fn parse_cards<'a, I>(lines: I) -> Vec<Card>
where
    I: IntoIterator<Item = DataLine<'a>>,
{
    let line_re = Regex::new(r"Card\s+(\d+): (.*) \| (.*)").unwrap();
    let nums_re = Regex::new(r"\d+").unwrap();

    let get_nums = |s: &str| {
        nums_re
            .find_iter(s)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    };

    let parse_card_opt = |line: &DataLine| {
        let caps = line_re.captures(line.line)?;
        let (_, [id, winning, ours]) = caps.extract();
        let id = id.parse().ok()?;

        let winning_nums = get_nums(winning);
        let nums_you_have = get_nums(ours);

        Some(Card {
            id,
            winning_nums,
            nums_you_have,
        })
    };

    let parse_card = |line: &DataLine| parse_card_opt(line).unwrap_or_else(|| panic!("{line}"));

    lines.into_iter().map(|line| parse_card(&line)).collect()
}

fn get_num_matches(card: Card) -> u32 {
    let mut winning_nums: HashSet<u64> = HashSet::new();
    card.winning_nums.iter().for_each(|&i| {
        winning_nums.insert(i);
    });
    card.nums_you_have
        .iter()
        .filter(|&i| winning_nums.contains(i))
        .count() as u32
}

fn doit(data: &String) -> u64 {
    let score_card = |card: Card| {
        let matches = get_num_matches(card);

        if matches == 0 {
            0
        } else {
            2u64.pow(matches - 1)
        }
    };

    let lines = get_non_empty_lines(data);
    let cards = parse_cards(lines);

    cards.into_iter().map(score_card).sum()
}

fn doit2(data: &String) -> u64 {
    let lines = get_non_empty_lines(data);
    let cards = parse_cards(lines);

    let mut id_to_num_owned: HashMap<u64, u64> = HashMap::new();

    cards.into_iter().for_each(|card| {
        let id = card.id;
        let num_owned = *id_to_num_owned
            .entry(id)
            .and_modify(|i| {
                *i += 1;
            })
            .or_insert(1);

        let num_matches = get_num_matches(card) as u64;

        let get_copies_of_card_ids = id + 1..(id + 1 + num_matches);

        get_copies_of_card_ids.for_each(|id| {
            id_to_num_owned
                .entry(id)
                .and_modify(|i| *i += num_owned)
                .or_insert(num_owned);
        })
    });

    id_to_num_owned.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day4/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 13);

        let answer = doit2(data);
        assert_eq!(answer, 30)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day4/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 20407);

        let answer = doit2(data);
        assert_eq!(answer, 23806951)
    }
}
