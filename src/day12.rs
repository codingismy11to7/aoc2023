use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::util::{get_non_empty_lines, DataLine};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct ParsedLine {
    first_part: String,
    nums: Vec<usize>,
}

fn parse_line(line: DataLine) -> ParsedLine {
    let (first, last) = line.line.split_once(' ').unwrap();

    let nums = NUM_RE
        .find_iter(last)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    ParsedLine {
        first_part: String::from(first),
        nums,
    }
}

fn count_valid_solutions(line: ParsedLine) -> u64 {
    fn rec(
        rem_str: &[char],
        rem_nums: &[usize],
        memo: &mut HashMap<(String, Vec<usize>), u64>,
    ) -> u64 {
        if let Some(a) = memo.get(&(String::from_iter(rem_str.iter()), rem_nums.to_vec())) {
            return *a;
        }

        if (rem_str.is_empty() || rem_str.iter().all(|&c| c != '#')) && rem_nums.is_empty() {
            1
        } else if rem_str.is_empty() || rem_nums.is_empty() {
            0
        } else {
            let curr_char = rem_str[0];
            if curr_char == '.' {
                rec(&rem_str[1..], rem_nums, memo)
            } else if curr_char == '?' {
                let mut a = rem_str.to_vec();
                a[0] = '.';
                let a = rec(&a, rem_nums, memo);

                let mut b = rem_str.to_vec();
                b[0] = '#';
                let b = rec(&b, rem_nums, memo);

                memo.insert(
                    (String::from_iter(rem_str.iter()), rem_nums.to_vec()),
                    a + b,
                );
                a + b
            } else {
                // char is #, let's see if the first number can go here
                let this_num = rem_nums[0];

                let can_fit = (0..this_num).all(|i| rem_str.get(i).is_some_and(|&c| c != '.'))
                    && !rem_str.get(this_num).is_some_and(|&c| c == '#');

                if can_fit {
                    // we are saying this one can fit, which means that the new start can only be
                    // '.' if it was a wildcard before
                    let mut next = rem_str[this_num..].to_vec();
                    if !next.is_empty() && next[0] == '?' {
                        next[0] = '.';
                    }
                    rec(&next, &rem_nums[1..], memo)
                } else {
                    memo.insert((String::from_iter(rem_str.iter()), rem_nums.to_vec()), 0);
                    0
                }
            }
        }
    }

    let str: Vec<_> = line.first_part.chars().collect();
    let mut memo = HashMap::new();

    rec(&str, &line.nums, &mut memo)
}

fn doit(data: &str) -> u64 {
    get_non_empty_lines(data)
        .map(parse_line)
        .map(count_valid_solutions)
        .sum()
}

fn doit2(data: &str) -> u64 {
    get_non_empty_lines(data)
        .map(parse_line)
        .map(|pl| {
            let first_part = vec![pl.first_part; 5].join("?");
            let nums = vec![pl.nums; 5].into_iter().flatten().collect();

            ParsedLine { first_part, nums }
        })
        .map(count_valid_solutions)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::{print_dur, read_file_panic};

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day12/test.txt");
        let answer = print_dur("test1", || doit(data));
        assert_eq!(answer, 21);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day12/test.txt");
        let answer = print_dur("test2", || doit2(data));
        assert_eq!(answer, 525152);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day12/data.txt");
        let answer = print_dur("real pt1", || doit(data));
        assert_eq!(answer, 7857);

        let answer = print_dur("real pt2", || doit2(data));
        assert_eq!(answer, 28606137449920);
    }
}
