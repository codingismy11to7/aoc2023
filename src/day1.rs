use crate::util::{get_non_empty_lines, DataLine};
use regex::Regex;

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn convert_num(num: &str) -> u64 {
    if num.len() == 1 {
        num.parse::<u64>().unwrap()
    } else {
        (1 + NUM_STRS.iter().position(|&s| s == num).unwrap()) as u64
    }
}

fn match_and_convert<'a, I>(re: &Regex, strs_to_test: I) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    convert_num(
        strs_to_test
            .into_iter()
            .find_map(|s| re.find(s))
            .unwrap()
            .as_str(),
    )
}

fn first_num(from: &str, re: &Regex) -> u64 {
    match_and_convert(re, (1..from.len() + 1).map(|i| &from[0..i]))
}

fn last_num(from: &str, re: &Regex) -> u64 {
    let len = from.len();
    match_and_convert(re, (0..len).rev().map(|i| &from[i..len]))
}

fn get_first_and_last_as_num(dl: &DataLine, re: &Regex) -> Option<u64> {
    let first = first_num(dl.line, re);
    let last = last_num(dl.line, re);
    let num_str = format!("{first}{last}");
    num_str.parse().ok()
}

fn doit_impl(data: &str, re: Regex) -> u64 {
    get_non_empty_lines(data).fold(0_u64, |acc, line| {
        acc + get_first_and_last_as_num(&line, &re).unwrap_or_else(|| panic!("{line}"))
    })
}

fn doit(data: &str) -> u64 {
    doit_impl(data, Regex::new(r"\d").unwrap())
}

fn build_str_num_regex() -> Regex {
    Regex::new(&format!("\\d|{}", NUM_STRS.join("|"))).unwrap()
}

fn doit2(data: &str) -> u64 {
    doit_impl(data, build_str_num_regex())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file_panic;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day1/part1/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 142)
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day1/part2/test.txt");
        let answer = doit2(data);
        assert_eq!(answer, 281)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day1/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 55002);

        let answer = doit2(data);
        assert_eq!(answer, 55093)
    }
}
