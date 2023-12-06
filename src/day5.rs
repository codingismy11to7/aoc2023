use rangemap::RangeMap;
use regex::Regex;
use std::ops::Range;

use crate::util::get_lines;

type Delta = i64;

#[derive(Debug)]
struct Map<'a> {
    #[allow(dead_code)]
    name: &'a str,
    range_map: RangeMap<i64, Delta>,
}
#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<i64>,
    maps: Vec<Map<'a>>,
}

fn default_range_map() -> RangeMap<i64, Delta> {
    let mut ret = RangeMap::new();
    ret.insert(0..i64::MAX, 0);
    ret
}

fn parse_almanac(data: &str) -> Almanac {
    let lines = get_lines(data);

    let seeds_re = Regex::new(r"seeds: (.*)").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    let map_header_re = Regex::new(r"(.+) map:").unwrap();
    let map_ent_re = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();

    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Map> = vec![];
    let mut curr_map: Option<Map> = None;

    lines.for_each(|line| {
        let to_num = |str: &str| -> i64 { str.parse().unwrap_or_else(|e| panic!("{line}, {e}")) };

        if line.line_number == 0 {
            let (_, [rest]) = seeds_re.captures(line.line).unwrap().extract();
            num_re
                .find_iter(rest)
                .for_each(|m| seeds.push(m.as_str().parse().unwrap()));
        } else if line.line_number > 1 {
            if line.line.trim().is_empty() {
                if let Some(m) = curr_map.take() {
                    maps.push(m);
                }
            } else {
                match map_header_re.captures(line.line) {
                    Some(cap) => {
                        let (_, [name]) = cap.extract();
                        curr_map = Some(Map {
                            name,
                            range_map: default_range_map(),
                        });
                    }

                    None => {
                        let (_, [dest, src, count]) = map_ent_re
                            .captures(line.line)
                            .unwrap_or_else(|| panic!("{line}"))
                            .extract();
                        let dest = to_num(dest);
                        let src = to_num(src);

                        let map = curr_map.as_mut().unwrap_or_else(|| panic!("{line}"));
                        map.range_map.insert(src..(src + to_num(count)), dest - src);
                    }
                }
            }
        }
    });

    Almanac { seeds, maps }
}

fn find_location(seed: i64, almanac: &Almanac) -> i64 {
    almanac.maps.iter().fold(seed, |src, map| {
        map.range_map.get(&src).map_or(src, |&delta| src + delta)
    })
}

fn doit(almanac: &Almanac) -> i64 {
    almanac.seeds.iter().fold(i64::MAX, |lowest, seed| {
        let loc = find_location(*seed, almanac);
        if loc < lowest {
            loc
        } else {
            lowest
        }
    })
}

fn lowest_location(ranges: Vec<Range<i64>>, rem: &[Map]) -> i64 {
    match rem.first() {
        None => ranges.iter().map(|r| r.start).min().unwrap(),
        Some(map) => {
            let new_ranges = ranges
                .iter()
                .flat_map(|r| {
                    map.range_map.overlapping(r).map(|(r2, &delta)| {
                        (r.start.max(r2.start) + delta)..(r.end.min(r2.end) + delta)
                    })
                })
                .collect();

            lowest_location(new_ranges, &rem[1..])
        }
    }
}

fn doit2(almanac: &Almanac) -> i64 {
    let seed_ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|start_len| start_len[0]..(start_len[0] + start_len[1]));

    seed_ranges.fold(i64::MAX, |lowest, seed| {
        let this_loc = lowest_location(vec![seed], &almanac.maps);

        if this_loc < lowest {
            this_loc
        } else {
            lowest
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::util::{print_dur, read_file_panic};

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day5/test.txt");
        let almanac = &parse_almanac(data);
        let answer = doit(almanac);
        assert_eq!(answer, 35);

        let answer = doit2(almanac);
        assert_eq!(answer, 46);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day5/data.txt");
        let almanac = &print_dur("parsed almanac", || parse_almanac(data));

        let answer = print_dur("did part 1", || doit(almanac));
        assert_eq!(answer, 178159714);

        let answer = print_dur("did part 2", || doit2(almanac));
        assert_eq!(answer, 100165128);
    }
}
