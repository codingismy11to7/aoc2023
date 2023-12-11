use crate::day10::Cardinal::{East, North, South, West};
use crate::util::get_non_empty_lines;

type XPos = usize;
type YPos = usize;
type Coord = (XPos, YPos);
type Grid = Vec<Vec<char>>;
type GridAndStartPoint = (Grid, Coord);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

fn north(c: Coord) -> Option<Coord> {
    let (x, y) = c;
    if y > 0 {
        Some((x, y - 1))
    } else {
        None
    }
}
fn south(c: Coord) -> Option<Coord> {
    let (x, y) = c;
    Some((x, y + 1))
}
fn west(c: Coord) -> Option<Coord> {
    let (x, y) = c;
    if x > 0 {
        Some((x - 1, y))
    } else {
        None
    }
}
fn east(c: Coord) -> Option<Coord> {
    let (x, y) = c;
    Some((x + 1, y))
}
fn neighbor(c: Coord, dir: Cardinal) -> Option<Coord> {
    match dir {
        North => north(c),
        South => south(c),
        East => east(c),
        West => west(c),
    }
}

fn pipe_for(dir_a: Cardinal, dir_b: Cardinal) -> char {
    match (dir_a, dir_b) {
        (North, South) | (South, North) => '|',
        (East, West) | (West, East) => '-',
        (North, East) | (East, North) => 'L',
        (North, West) | (West, North) => 'J',
        (South, West) | (West, South) => '7',
        (South, East) | (East, South) => 'F',
        (_, _) => {
            panic!("couldn't find pipe")
        }
    }
}
fn connects(c: char) -> Option<(Cardinal, Cardinal)> {
    match c {
        '|' => Some((North, South)),
        '-' => Some((East, West)),
        'L' => Some((North, East)),
        'J' => Some((North, West)),
        '7' => Some((South, West)),
        'F' => Some((South, East)),
        _ => None,
    }
}

fn opposite(dir: Cardinal) -> Cardinal {
    match dir {
        North => South,
        South => North,
        East => West,
        West => East,
    }
}

fn connects_to(c: char, dir: Cardinal) -> bool {
    connects(c).iter().any(|(a, b)| *a == dir || *b == dir)
}

fn char_at(grid: &Grid, coord: Coord) -> Option<char> {
    let (x, y) = coord;
    let line = grid.get(y)?;
    let c = line.get(x)?;
    Some(*c)
}

fn replace_start_point((mut grid, sp): GridAndStartPoint) -> GridAndStartPoint {
    let at_north = north(sp).into_iter().flat_map(|c| char_at(&grid, c)).next();
    let at_south = south(sp).into_iter().flat_map(|c| char_at(&grid, c)).next();
    let at_east = east(sp).into_iter().flat_map(|c| char_at(&grid, c)).next();
    let at_west = west(sp).into_iter().flat_map(|c| char_at(&grid, c)).next();

    let get_connects = |c: Option<char>, d| c.filter(|&c| connects_to(c, d)).map(|_| opposite(d));

    let north_connects = get_connects(at_north, South);
    let south_connects = get_connects(at_south, North);
    let east_connects = get_connects(at_east, West);
    let west_connects = get_connects(at_west, East);

    let connections = vec![north_connects, south_connects, east_connects, west_connects]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let a = &connections[0];
    let b = &connections[1];

    let pipe = pipe_for(*a, *b);

    let (x, y) = sp;
    grid[y][x] = pipe;

    (grid, sp)
}

fn parse_grid(data: &str) -> GridAndStartPoint {
    replace_start_point(
        get_non_empty_lines(data).fold((vec![], (0, 0)), |acc, line| {
            let (mut grid, sp) = acc;

            let this_line = line.line.chars().collect::<Vec<_>>();

            let new_sp_opt = (0..this_line.len())
                .find(|&i| this_line[i] == 'S')
                .map(|x| (x, line.line_number));

            grid.push(this_line);

            (grid, new_sp_opt.unwrap_or(sp))
        }),
    )
}

type Step = u64;

/**
 * calls the callback with each non-starting-point coordinate and how many steps it is
 * from the starting point
 */
fn traverse_loop<F>(grid: &Grid, start_point: Coord, init_dir: Cardinal, mut callback: F)
where
    F: FnMut(Coord, Step) -> bool,
{
    let mut curr_dir = init_dir;
    let mut curr_coord = neighbor(start_point, init_dir).unwrap();
    let mut steps = 1;

    loop {
        if curr_coord == start_point {
            break;
        }

        let should_break = callback(curr_coord, steps);

        if should_break {
            break;
        }

        let this_pipe = char_at(grid, curr_coord).unwrap();
        let (dirs_a, dirs_b) = connects(this_pipe).unwrap();

        curr_dir = if opposite(dirs_a) == curr_dir {
            dirs_b
        } else {
            dirs_a
        };
        curr_coord = neighbor(curr_coord, curr_dir).unwrap();
        steps += 1;
    }
}

fn doit(data: &GridAndStartPoint) -> u64 {
    let (grid, sp) = data;
    let sp = *sp;

    let line_len = grid[0].len();
    let line_count = grid.len();

    let mut distance_grid: Vec<_> = (0..line_count).map(|_| vec![0; line_len]).collect();

    let (dir_a, dir_b) = connects(char_at(grid, sp).unwrap()).unwrap();

    let mut fill_in_steps = |init_dir: Cardinal| {
        traverse_loop(grid, sp, init_dir, |curr_coord, which_step| {
            let (x, y) = curr_coord;

            let curr_value_in_grid = distance_grid[y][x];
            if curr_value_in_grid == 0 || which_step < curr_value_in_grid {
                distance_grid[y][x] = which_step;
                false
            } else {
                true
            }
        });
    };

    fill_in_steps(dir_a);

    fill_in_steps(dir_b);

    distance_grid.into_iter().flatten().max().unwrap()
}

fn doit2(data: &GridAndStartPoint) -> i64 {
    let (grid, sp) = data;
    let sp = *sp;

    let mut edge_coords = vec![sp];

    let (dir, _) = connects(char_at(grid, sp).unwrap()).unwrap();
    traverse_loop(grid, sp, dir, |c, _| {
        edge_coords.push(c);

        false
    });

    let num_of_boundary_points = edge_coords.len();

    edge_coords.push(sp);

    // definitely had to scroll through aoc reddit to find somebody mentioning shoelace formula
    // to find the area of a polygon, and pick's theorem to find the area given number of
    // internal points & number of boundary points (to rearrange and solve for interior, given
    // we find the area first)

    let area = ((0..num_of_boundary_points).fold(0i64, |acc, i| {
        let (x1, y1) = edge_coords[i];
        let (x2, y2) = edge_coords[i + 1];

        acc + (x1 as i64 * y2 as i64) - (y1 as i64 * x2 as i64)
    }) / 2)
        .abs();

    area + 1 - (num_of_boundary_points as i64 / 2)
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day10/part1/test1.txt");
        let data = &parse_grid(data);
        let answer = doit(data);
        assert_eq!(answer, 4);

        let data = &read_file_panic("./data/day10/part1/test2.txt");
        let data = &parse_grid(data);
        let answer = doit(data);
        assert_eq!(answer, 8);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day10/part2/test1.txt");
        let data = &parse_grid(data);
        let answer = doit2(data);
        assert_eq!(answer, 4);

        let data = &read_file_panic("./data/day10/part2/test2.txt");
        let data = &parse_grid(data);
        let answer = doit2(data);
        assert_eq!(answer, 8);

        let data = &read_file_panic("./data/day10/part2/test3.txt");
        let data = &parse_grid(data);
        let answer = doit2(data);
        assert_eq!(answer, 10);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day10/data.txt");
        let data = &parse_grid(data);
        let answer = doit(data);
        assert_eq!(answer, 6907);

        let answer = doit2(data);
        assert_eq!(answer, 541);
    }
}
