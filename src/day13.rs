use tailcall::tailcall;

use crate::util::get_lines;

#[derive(Clone)]
struct BlockInfo {
    num_rows: usize,
    num_cols: usize,
}
struct Block {
    data: Vec<Vec<char>>,
    info: BlockInfo,
}
struct NumBlock {
    cols: Vec<u64>,
    rows: Vec<u64>,
}
enum MirrorPoint {
    Column(usize),
    Row(usize),
}

impl MirrorPoint {
    fn value_if(&self, is_col: bool) -> Option<usize> {
        match self {
            MirrorPoint::Column(c) => {
                if is_col {
                    Some(*c)
                } else {
                    None
                }
            }
            MirrorPoint::Row(r) => {
                if is_col {
                    None
                } else {
                    Some(*r)
                }
            }
        }
    }
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

fn block_to_numblock(block: &Block) -> NumBlock {
    fn block_col(block: &Block, col: usize) -> impl Iterator<Item = &char> {
        (0..block.data.len()).map(move |i| &block.data[i][col])
    }

    let rows = block
        .data
        .iter()
        .map(|line| convert_to_number(line.iter()))
        .collect();
    let cols = (0..block.data[0].len())
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

fn block_to_smudge_corrected(block: &Block) -> Vec<Block> {
    let mut ret = vec![];

    (0..block.info.num_rows).for_each(|row| {
        (0..block.info.num_cols).for_each(|col| {
            let mut data = block.data.clone();
            data[row][col] = if data[row][col] == '.' { '#' } else { '.' };
            ret.push(Block {
                data,
                info: block.info.clone(),
            })
        })
    });

    ret
}

fn find_mirrorpoint(nums: &Vec<u64>, but_not: Option<usize>) -> Option<usize> {
    (0..nums.len() - 1)
        .filter(|i| !but_not.iter().any(|j| j == i))
        .find(|&mirror| is_mirrorpoint(nums, mirror))
}

fn numblock_to_mirrorpoint_opt(
    but_not: &Option<MirrorPoint>,
    block: NumBlock,
) -> Option<MirrorPoint> {
    find_mirrorpoint(
        &block.rows,
        but_not.iter().flat_map(|o| o.value_if(false)).next(),
    )
    .map(MirrorPoint::Row)
    .or_else(|| {
        find_mirrorpoint(
            &block.cols,
            but_not.iter().flat_map(|o| o.value_if(true)).next(),
        )
        .map(MirrorPoint::Column)
    })
}

fn numblock_to_mirrorpoint(but_not: &Option<MirrorPoint>, block: NumBlock) -> MirrorPoint {
    numblock_to_mirrorpoint_opt(but_not, block).unwrap()
}

fn mirrorpoint_to_num(mp: MirrorPoint) -> u64 {
    match mp {
        MirrorPoint::Column(c) => (c as u64) + 1,
        MirrorPoint::Row(r) => 100 * ((r as u64) + 1),
    }
}

fn parse_blocks(data: &str) -> Vec<Block> {
    let mut blocks = vec![];

    fn block_from(v: Vec<Vec<char>>) -> Block {
        let info = BlockInfo {
            num_rows: v.len(),
            num_cols: v[0].len(),
        };
        Block { data: v, info }
    }

    let last_opt = get_lines(data).fold(None, |acc, line| {
        if line.line.trim().is_empty() {
            if let Some(prev) = acc {
                blocks.push(block_from(prev));
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
        blocks.push(block_from(last))
    }

    blocks
}

fn doit(data: &str) -> u64 {
    parse_blocks(data)
        .iter()
        .map(block_to_numblock)
        .map(|b| numblock_to_mirrorpoint(&None, b))
        .map(mirrorpoint_to_num)
        .sum()
}

fn doit2(data: &str) -> u64 {
    parse_blocks(data)
        .iter()
        .map(|block| {
            let old_mirror_point = &numblock_to_mirrorpoint_opt(&None, block_to_numblock(block));

            block_to_smudge_corrected(block)
                .iter()
                .map(block_to_numblock)
                .filter_map(|b| numblock_to_mirrorpoint_opt(old_mirror_point, b))
                .next()
                .unwrap()
        })
        .map(mirrorpoint_to_num)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::{print_dur, read_file_panic};

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
        assert_eq!(answer, 400);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day13/data.txt");
        let answer = print_dur("part1", || doit(data));
        assert_eq!(answer, 29213);

        let answer = print_dur("part2", || doit2(data));
        assert_eq!(answer, 37453);
    }
}
