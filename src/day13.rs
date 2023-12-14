use tailcall::tailcall;

use crate::util::get_lines;

type Block = Vec<Vec<char>>;
struct NumBlock {
    cols: Vec<u64>,
    rows: Vec<u64>,
}
enum MirrorPoint {
    Column(u64),
    Row(u64),
}

fn convert_to_number<'a>(chars: impl Iterator<Item = &'a char>) -> u64 {
    #[tailcall]
    fn rec<'a>(acc: u64, mult: u64, mut chars: impl Iterator<Item = &'a char>) -> u64 {
        match chars.next() {
            None => acc,
            Some(&c) => {
                let this_bit = if c == '.' { 0 } else { 1 };
                rec(acc + this_bit * mult, 2 * mult, chars)
            }
        }
    }

    rec(0, 1, chars)
}

fn block_to_numblock(block: Block) -> NumBlock {
    fn block_col(block: &Block, col: usize) -> impl Iterator<Item = &char> {
        (0..block.len()).map(move |i| &block[i][col])
    }

    let rows = block
        .iter()
        .map(|line| convert_to_number(line.iter()))
        .collect();
    let cols = (0..block[0].len())
        .map(|col| block_col(&block, col))
        .map(convert_to_number)
        .collect();

    NumBlock { rows, cols }
}

fn is_mirrorpoint(nums: &Vec<u64>, mirror_after: usize) -> bool {
    #[tailcall]
    fn rec(nums: &Vec<u64>, mirror_after: usize, delta: usize) -> bool {
        let left = if (delta - 1) > mirror_after {
            None
        } else {
            nums.get(mirror_after - (delta - 1))
        };
        let right = nums.get(mirror_after + delta);

        match (left, right) {
            (Some(l), Some(r)) => {
                if l != r {
                    false
                } else {
                    rec(nums, mirror_after, 1 + delta)
                }
            }
            (_, _) => true,
        }
    }

    rec(nums, mirror_after, 1)
}

fn find_mirrorpoint(nums: &Vec<u64>) -> Option<usize> {
    (0..nums.len() - 1).find(|&mirror| is_mirrorpoint(nums, mirror))
}

fn numblock_to_mirrorpoint_opt(block: NumBlock) -> Option<MirrorPoint> {
    find_mirrorpoint(&block.rows)
        .map(|i| MirrorPoint::Row(i as u64))
        .or_else(|| find_mirrorpoint(&block.cols).map(|i| MirrorPoint::Column(i as u64)))
}

fn numblock_to_mirrorpoint(block: NumBlock) -> MirrorPoint {
    numblock_to_mirrorpoint_opt(block).unwrap()
}

fn mirrorpoint_to_num(mp: MirrorPoint) -> u64 {
    match mp {
        MirrorPoint::Column(c) => c + 1,
        MirrorPoint::Row(r) => 100 * (r + 1),
    }
}

fn parse_blocks(data: &str) -> Vec<Block> {
    let mut blocks = vec![];

    let last_opt = get_lines(data).fold(None, |acc, line| {
        if line.line.trim().is_empty() {
            if let Some(prev) = acc {
                blocks.push(prev);
            }
            None
        } else {
            let this_line = line.line.chars().collect();
            match acc {
                None => Some(vec![this_line]),
                Some(mut curr_block) => {
                    curr_block.push(this_line);
                    Some(curr_block)
                }
            }
        }
    });

    if let Some(last) = last_opt {
        blocks.push(last)
    }

    blocks
}

fn doit(data: &str) -> u64 {
    parse_blocks(data)
        .into_iter()
        .map(block_to_numblock)
        .map(numblock_to_mirrorpoint)
        .map(mirrorpoint_to_num)
        .sum()
}

fn doit2(data: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day13/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 405);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day13/test.txt");
        let answer = doit2(data);
        assert_eq!(answer, 0);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day13/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 29213);

        let answer = doit2(data);
        assert_eq!(answer, 0);
    }
}
