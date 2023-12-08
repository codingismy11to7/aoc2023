use crate::util::get_non_empty_lines;
use regex::Regex;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;
use tailcall::tailcall;

#[derive(Debug)]
struct Elems<'a> {
    left: &'a str,
    right: &'a str,
}
#[derive(Debug)]
struct Map<'a> {
    instructions: &'a str,
    nodes: HashMap<&'a str, Elems<'a>>,
}

fn parse_map(data: &str) -> Map {
    let elems_re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();

    let mut lines = get_non_empty_lines(data);

    let instructions = lines.next().unwrap().line;

    let nodes: HashMap<_, _> = lines
        .map(|line| {
            let (_, [label, left, right]) = elems_re
                .captures(line.line)
                .unwrap_or_else(|| panic!("{line}"))
                .extract();
            let elems = Elems { left, right };

            (label, elems)
        })
        .collect();

    Map {
        instructions,
        nodes,
    }
}

fn get_num_steps<F>(map: &Map, start: &str, is_end_point: F) -> u64
where
    F: Fn(&str) -> bool,
{
    #[tailcall]
    fn count_steps<F>(
        map: &Map,
        is_end_point: F,
        curr: &str,
        acc: u64,
        mut instructions: Cycle<Chars>,
    ) -> u64
    where
        F: Fn(&str) -> bool,
    {
        if is_end_point(curr) {
            acc
        } else {
            let next = instructions.next().unwrap();
            let this = map.nodes.get(curr).unwrap();

            count_steps(
                map,
                is_end_point,
                if next == 'R' { this.right } else { this.left },
                1 + acc,
                instructions,
            )
        }
    }

    count_steps(
        map,
        is_end_point,
        start,
        0,
        map.instructions.chars().cycle(),
    )
}

fn doit(map: &Map) -> u64 {
    get_num_steps(map, "AAA", |curr| curr.eq("ZZZ"))
}

#[tailcall]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn doit2(map: &Map) -> u64 {
    let starts = map
        .nodes
        .keys()
        .filter_map(|&k| Some(k).filter(|k| k.ends_with('A')));

    let lens = starts.map(|s| get_num_steps(map, s, |n| n.ends_with('Z')));

    lens.reduce(lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day8/part1/test1.txt");
        let map = &parse_map(data);
        let answer = doit(map);
        assert_eq!(answer, 2);

        let data = &read_file_panic("./data/day8/part1/test2.txt");
        let map = &parse_map(data);
        let answer = doit(map);
        assert_eq!(answer, 6);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day8/part2/test.txt");
        let map = &parse_map(data);
        let answer = doit2(map);
        assert_eq!(answer, 6);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day8/data.txt");
        let map = &parse_map(data);
        let answer = doit(map);
        assert_eq!(answer, 11309);

        let answer = doit2(map);
        assert_eq!(answer, 13740108158591);
    }
}
