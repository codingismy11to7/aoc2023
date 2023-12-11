use std::collections::HashSet;

use crate::util::get_non_empty_lines;

type XPos = i64;
type YPos = i64;
type Coord = (XPos, YPos);
struct PictureData {
    galaxy_coords: Vec<Coord>,
    cols_with_no_galaxies: HashSet<usize>,
    rows_with_no_galaxies: HashSet<usize>,
}

fn get_picture_data(data: &str) -> PictureData {
    let mut galaxy_coords = Vec::new();
    let mut cols_with_no_galaxies = HashSet::new();
    let mut rows_with_no_galaxies = HashSet::new();

    get_non_empty_lines(data).for_each(|line| {
        let mut has_galaxy = false;
        line.line.chars().enumerate().for_each(|(col, char)| {
            if char == '#' {
                has_galaxy = true;
                cols_with_no_galaxies.remove(&col);
                galaxy_coords.push((col as i64, line.line_number as i64));
            } else if line.line_number == 0 {
                // on the first line, put all the columns with empty space so we can remove them later
                cols_with_no_galaxies.insert(col);
            }
        });
        if !has_galaxy {
            rows_with_no_galaxies.insert(line.line_number);
        }
    });

    PictureData {
        galaxy_coords,
        cols_with_no_galaxies,
        rows_with_no_galaxies,
    }
}

fn expand_universe(picture: &PictureData, expansion_factor: u32) -> Vec<Coord> {
    picture
        .galaxy_coords
        .iter()
        .map(|(x, y)| {
            let num_of_empty_cols_before = (expansion_factor as i64 - 1)
                * picture
                    .cols_with_no_galaxies
                    .iter()
                    .filter(|&&c| (c as i64) < *x)
                    .count() as i64;
            let num_of_empty_rows_before = (expansion_factor as i64 - 1)
                * picture
                    .rows_with_no_galaxies
                    .iter()
                    .filter(|&&r| (r as i64) < *y)
                    .count() as i64;

            (x + num_of_empty_cols_before, y + num_of_empty_rows_before)
        })
        .collect()
}

fn doit_impl(data: &PictureData, expansion_factor: u32) -> i64 {
    let coords = expand_universe(data, expansion_factor);

    fn distance_between((x1, y1): Coord, (x2, y2): Coord) -> i64 {
        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();

        dx + dy
    }

    (0..coords.len())
        .flat_map(|i| {
            (i + 1..coords.len())
                .map(|j| distance_between(coords[i], coords[j]))
                .collect::<Vec<_>>()
        })
        .sum()
}

fn doit(data: &PictureData) -> i64 {
    doit_impl(data, 2)
}

fn doit2(data: &PictureData) -> i64 {
    doit_impl(data, 1_000_000)
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t1() {
        let data = &read_file_panic("./data/day11/test.txt");
        let data = &get_picture_data(data);
        let answer = doit(data);
        assert_eq!(answer, 374);
    }

    #[test]
    fn t2() {
        let data = &read_file_panic("./data/day11/test.txt");
        let data = &get_picture_data(data);
        let answer = doit_impl(data, 10);
        assert_eq!(answer, 1030);

        let answer = doit_impl(data, 100);
        assert_eq!(answer, 8410);
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day11/data.txt");
        let data = &get_picture_data(data);
        let answer = doit(data);
        assert_eq!(answer, 9274989);

        let answer = doit2(data);
        assert_eq!(answer, 357134560737);
    }
}
