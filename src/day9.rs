use regex::{Match, Regex};

use crate::util::get_non_empty_lines;

fn calculate_impl(history: Vec<i64>, next: bool) -> i64 {
    let mut seqs = vec![history];

    fn get_diffs(v: &Vec<i64>) -> Vec<i64> {
        (0..(v.len() - 1)).map(|idx| v[idx + 1] - v[idx]).collect()
    }

    let mut i = 0;
    loop {
        let diffs = get_diffs(&seqs[i]);
        if diffs.iter().all(|&x| x == 0) {
            break;
        } else {
            seqs.push(diffs);
            i += 1;
        }
    }

    let mut curr = 0;
    loop {
        if next {
            curr += *seqs[i].last().unwrap();
        } else {
            curr = *seqs[i].first().unwrap() - curr;
        }
        if i == 0 {
            break;
        } else {
            i -= 1
        }
    }

    curr
}

fn parse_lines(data: &str) -> Vec<Vec<i64>> {
    let lines = get_non_empty_lines(data);

    let num_re = Regex::new(r"-?\d+").unwrap();
    let i = |m: Match| m.as_str().parse().ok();

    lines
        .map(|line| {
            num_re
                .find_iter(line.line)
                .map(i)
                .map(|opt| opt.unwrap_or_else(|| panic!("{line}")))
                .collect()
        })
        .collect()
}

fn doit_impl(data: &str, next: bool) -> i64 {
    parse_lines(data)
        .into_iter()
        .map(|x| calculate_impl(x, next))
        .sum()
}

fn doit(data: &str) -> i64 {
    doit_impl(data, true)
}

fn doit2(data: &str) -> i64 {
    doit_impl(data, false)
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day9/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 114);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day9/test.txt");
        let answer = doit2(data);
        assert_eq!(answer, 2);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day9/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 1696140818);

        let answer = doit2(data);
        assert_eq!(answer, 1152);
    }
}
