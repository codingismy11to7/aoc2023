use crate::util::{get_non_empty_lines, DataLine};
use regex::Regex;
use std::collections::HashMap;

fn slide_north(board: &mut Vec<Vec<char>>) {
    (0..board[0].len()).for_each(|col| {
        let mut min_row = 0;
        loop {
            match (min_row..board.len()).find(|&r| board[r][col] == 'O') {
                None => break,
                Some(curr_o_row) => {
                    if curr_o_row == 0 || board[curr_o_row - 1][col] != '.' {
                        min_row = curr_o_row + 1;
                    } else {
                        board[curr_o_row - 1][col] = 'O';
                        board[curr_o_row][col] = '.';
                    }
                }
            }
        }
    })
}
fn slide_west(board: &mut Vec<Vec<char>>) {
    (0..board.len()).for_each(|row| {
        let mut min_col = 0;
        loop {
            match (min_col..board[0].len()).find(|&c| board[row][c] == 'O') {
                None => break,
                Some(curr_o_col) => {
                    if curr_o_col == 0 || board[row][curr_o_col - 1] != '.' {
                        min_col = curr_o_col + 1
                    } else {
                        board[row][curr_o_col - 1] = 'O';
                        board[row][curr_o_col] = '.';
                    }
                }
            }
        }
    });
}
fn slide_south(board: &mut Vec<Vec<char>>) {
    let board_row_count = board.len();
    let last_row_idx = board_row_count - 1;
    (0..board[0].len()).for_each(|col| {
        let mut max_row = board_row_count - 1;
        loop {
            match (0..max_row).rev().find(|&r| board[r][col] == 'O') {
                None => break,
                Some(curr_o_row) => {
                    if curr_o_row == last_row_idx || board[curr_o_row + 1][col] != '.' {
                        max_row -= 1;
                    } else {
                        board[curr_o_row + 1][col] = 'O';
                        board[curr_o_row][col] = '.';
                    }
                }
            }
        }
    })
}
fn slide_east(board: &mut Vec<Vec<char>>) {
    let board_col_count = board[0].len();
    let last_col_idx = board_col_count - 1;
    (0..board.len()).for_each(|row| {
        let mut max_col = board_col_count - 1;
        loop {
            match (0..max_col).rev().find(|&c| board[row][c] == 'O') {
                None => break,
                Some(curr_o_col) => {
                    if curr_o_col == last_col_idx || board[row][curr_o_col + 1] != '.' {
                        max_col -= 1;
                    } else {
                        board[row][curr_o_col + 1] = 'O';
                        board[row][curr_o_col] = '.';
                    }
                }
            }
        }
    })
}

fn run_cycle<'a>(
    board: &'a Vec<Vec<char>>,
    mut memo: HashMap<&'a Vec<Vec<char>>, Vec<Vec<char>>>,
) -> (Vec<Vec<char>>, HashMap<&'a Vec<Vec<char>>, Vec<Vec<char>>>) {
    match memo.get(board) {
        Some(b) => (b.clone(), memo),
        None => {
            let mut out_board = board.clone();
            slide_north(&mut out_board);
            slide_west(&mut out_board);
            slide_south(&mut out_board);
            slide_east(&mut out_board);
            let map_val = out_board.clone();
            memo.insert(board, map_val);
            (out_board, memo)
        }
    }
}

fn get_load(board: Vec<Vec<char>>) -> u64 {
    let num_rows = board.len();
    board
        .into_iter()
        .enumerate()
        .map(|(row_num, line)| {
            ((num_rows - row_num) as u64) * line.into_iter().filter(|&c| c == 'O').count() as u64
        })
        .sum()
}

fn doit(data: &str) -> u64 {
    let mut board = get_non_empty_lines(data)
        .map(|dl| dl.line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    slide_north(&mut board);

    get_load(board)
}

fn print_board(desc: &str, board: &Vec<Vec<char>>) {
    println!("{desc}:");
    board.iter().for_each(|l| {
        println!(
            "{}",
            l.iter()
                .map(|&c| format!("{c}"))
                .collect::<Vec<_>>()
                .join("")
        )
    });
    println!("==========");
}

fn doit2(data: &str) -> u64 {
    // need to find when it loops to an older one, find out the number between loops,
    // shorten the number of cycles by modulo that, then find the answer. because this
    // dumb version doesn't finish

    let mut board = get_non_empty_lines(data)
        .map(|dl| dl.line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut memo = HashMap::new();
    let (board, _) = (0..1000000000).fold((board.clone(), memo), |(old_board, memo), _| {
        run_cycle(&board, memo)
    });
    // (0..1000000000).for_each(|_| run_cycle(&board));

    get_load(board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file_panic;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day14/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 136);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day14/test.txt");
        let answer = doit2(data);
        assert_eq!(answer, 64);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day14/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 106186);

        let answer = doit2(data);
        assert_eq!(answer, 0);
    }
}
