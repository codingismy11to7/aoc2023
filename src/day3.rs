use crate::util::{get_non_empty_lines, DataLine};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Hash)]
struct PartNum {
    line_num: usize,
    indices: Range<usize>,
    num: u64,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    line_num: usize,
    idx: usize,
}

fn surrounding_coords(line_num: usize, indices: &Range<usize>) -> Vec<Coord> {
    let start = indices.start;
    let end = indices.end;

    let mut ret: Vec<Coord> = vec![];

    let mut add_above_or_below = |line_num: usize| {
        let start_idx = if start == 0 { 0 } else { start - 1 };
        (start_idx..end + 1).for_each(|idx| ret.push(Coord { line_num, idx }));
    };

    if line_num > 0 {
        add_above_or_below(line_num - 1);
    }
    add_above_or_below(line_num + 1);

    if start > 0 {
        ret.push(Coord {
            line_num,
            idx: start - 1,
        })
    }
    ret.push(Coord { line_num, idx: end });

    ret
}

fn part_nums_from_lines<'a, I>(lines: I) -> Vec<PartNum>
where
    I: Iterator<Item = &'a DataLine<'a>>,
{
    let num_re = Regex::new(r"\d+").unwrap();

    let part_nums_from_line = |line: &DataLine| {
        num_re
            .find_iter(line.line)
            .map(|m| PartNum {
                line_num: line.line_number,
                indices: m.range(),
                num: m.as_str().parse().unwrap_or_else(|_| panic!("{line}")),
            })
            .collect::<Vec<_>>()
    };

    lines.flat_map(part_nums_from_line).collect()
}

fn doit(data: &String) -> u64 {
    let lines = get_non_empty_lines(data).collect::<Vec<_>>();

    let part_nums = part_nums_from_lines(lines.iter());

    let two_d_board = lines
        .iter()
        .map(|line| line.line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let char_at = |c: &Coord| {
        let line = two_d_board.get(c.line_num)?;
        let &char = line.get(c.idx)?;
        Some(char)
    };

    let adjacent_to_symbol = |part_num: &PartNum| {
        surrounding_coords(part_num.line_num, &part_num.indices)
            .iter()
            .any(|coord| char_at(coord).is_some_and(|c| !c.is_ascii_digit() && (c != '.')))
    };

    part_nums
        .iter()
        .filter(|&pn| adjacent_to_symbol(pn))
        .fold(0, |acc, pn| acc + pn.num)
}

fn doit2(data: &String) -> u64 {
    let lines = get_non_empty_lines(data).collect::<Vec<_>>();

    let part_nums = part_nums_from_lines(lines.iter());

    let mut coords_to_pns: HashMap<Coord, &PartNum> = HashMap::new();
    part_nums.iter().for_each(|pn| {
        pn.indices.clone().for_each(|idx| {
            coords_to_pns.insert(
                Coord {
                    line_num: pn.line_num,
                    idx,
                },
                pn,
            );
        })
    });

    let mut total = 0u64;

    lines.iter().for_each(|line| {
        line.line.char_indices().for_each(|(idx, char)| {
            if char == '*' {
                let touching = surrounding_coords(line.line_number, &(idx..idx + 1));
                let mut parts: HashSet<&PartNum> = HashSet::new();
                touching.iter().for_each(|c| {
                    coords_to_pns.get(c).iter().for_each(|pn| {
                        parts.insert(pn);
                    });
                });
                if parts.len() == 2 {
                    total += parts.iter().fold(1, |acc, pn| acc * pn.num)
                }
            }
        })
    });

    total
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day3/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 4361);

        let answer = doit2(data);
        assert_eq!(answer, 467835);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day3/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 535235);

        let answer = doit2(data);
        assert_eq!(answer, 79844424)
    }
}
