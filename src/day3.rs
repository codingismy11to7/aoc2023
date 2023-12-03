use crate::util::{get_non_empty_lines, DataLine};

#[derive(Default, Clone)]
struct PartNum {
    line_num: usize,
    start_idx: usize,
    end_idx: usize,
    num: u64,
}

struct NumBeingParsed {
    start_idx: usize,
    chars: String,
}

fn doit(data: &String) -> u64 {
    let lines = get_non_empty_lines(data).collect::<Vec<_>>();

    let mut part_nums = vec![
        PartNum {
            ..Default::default()
        };
        0
    ];

    let mut add_part_num = |line: &DataLine, being_parsed: NumBeingParsed, end_idx: usize| {
        part_nums.push(PartNum {
            line_num: line.line_number,
            start_idx: being_parsed.start_idx,
            end_idx,
            num: being_parsed
                .chars
                .parse()
                .unwrap_or_else(|_| panic!("{line}")),
        });
    };

    lines.iter().for_each(|line| {
        let was_being_parsed =
            line.line
                .char_indices()
                .fold(None::<NumBeingParsed>, |curr_parsing, (idx, char)| {
                    if char.is_ascii_digit() {
                        curr_parsing
                            .map(|being_parsed| NumBeingParsed {
                                chars: format!("{}{}", being_parsed.chars, char),
                                ..being_parsed
                            })
                            .or_else(|| {
                                Some(NumBeingParsed {
                                    start_idx: idx,
                                    chars: String::from(char),
                                })
                            })
                    } else {
                        match curr_parsing {
                            None => curr_parsing,
                            Some(being_parsed) => {
                                add_part_num(line, being_parsed, idx - 1);
                                None
                            }
                        }
                    }
                });

        if let Some(being_parsed) = was_being_parsed {
            add_part_num(line, being_parsed, line.line.len() - 1);
        }
    });

    let two_d_board = lines
        .iter()
        .map(|line| line.line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let char_at = |line_num: usize, idx: usize| {
        let line = two_d_board.get(line_num - 1)?;
        let &char = line.get(idx)?;
        Some(char)
    };

    0
}

fn doit2(data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day3/part1/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 4361)
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day3/part2/test.txt");
        let answer = doit2(data);
        assert_eq!(answer, 0)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day3/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 0);

        let answer = doit2(data);
        assert_eq!(answer, 0)
    }
}
