use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Formatter;

use regex::Regex;

use crate::util::{get_non_empty_lines, DataLine};

trait Parse {
    fn game_num_and_rest<'a>(&self, line: &'a str) -> Result<(u64, &'a str), Box<dyn Error>>;
    fn split_semicolons<'h>(&self, from: &'h str) -> Vec<&'h str>;
    fn split_commas<'h>(&self, from: &'h str) -> Vec<&'h str>;
    fn get_num_and_color<'a>(&self, text: &'a str) -> Result<(u64, &'a str), Box<dyn Error>>;
}
struct Regexes {
    game: Regex,
    semi: Regex,
    comma: Regex,
    num_and_color: Regex,
}
impl Parse for Regexes {
    fn game_num_and_rest<'a>(&self, line: &'a str) -> Result<(u64, &'a str), Box<dyn Error>> {
        let (_, [game_num, rest]) = self.game.captures(line).ok_or(ParseError)?.extract();
        let game_num: u64 = game_num.parse()?;
        Ok((game_num, rest))
    }

    fn split_semicolons<'h>(&self, from: &'h str) -> Vec<&'h str> {
        self.semi.split(from).collect()
    }

    fn split_commas<'h>(&self, from: &'h str) -> Vec<&'h str> {
        self.comma.split(from).collect()
    }

    fn get_num_and_color<'a>(&self, text: &'a str) -> Result<(u64, &'a str), Box<dyn Error>> {
        let (_, [num, color]) = self
            .num_and_color
            .captures(text)
            .ok_or(ParseError)?
            .extract();
        let num: u64 = num.parse()?;
        Ok((num, color))
    }
}

fn create_parse() -> Result<Regexes, Box<dyn Error>> {
    let game = Regex::new(r"Game (\d+): (.*)")?;
    let semi = Regex::new(r"\s*;\s*")?;
    let comma = Regex::new(r"\s*,\s*")?;
    let num_and_color = Regex::new(r"(\d+)\s+(.*)")?;

    Ok(Regexes {
        game,
        semi,
        comma,
        num_and_color,
    })
}

type Color = str;
type Count = u64;
type Draw<'a> = HashMap<&'a Color, Count>;
type DiceCounts<'a> = Draw<'a>;

#[derive(Debug)]
struct Game<'a> {
    id: u64,
    draws: Vec<Draw<'a>>,
}

#[derive(Debug)]
struct ParseError;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing")
    }
}
impl Error for ParseError {}

fn parse_game<'a>(line: &DataLine<'a>, parser: &impl Parse) -> Result<Game<'a>, Box<dyn Error>> {
    let (game_num, rest) = parser.game_num_and_rest(line.line)?;

    let draws = parser
        .split_semicolons(rest)
        .iter()
        .map(|info| {
            parser
                .split_commas(info)
                .iter()
                .try_fold(HashMap::new(), |mut acc, draw| {
                    let (count, color) = parser.get_num_and_color(draw)?;
                    acc.insert(color, count);
                    Ok(acc)
                })
        })
        .collect::<Result<Vec<Draw>, Box<dyn Error>>>()?;

    Ok(Game {
        id: game_num,
        draws,
    })
}

fn parse_games<'a, I>(lines: I) -> Result<Vec<Game<'a>>, Box<dyn Error>>
where
    I: IntoIterator<Item = DataLine<'a>>,
{
    let parser = create_parse()?;
    let games = lines.into_iter().map(|line| {
        parse_game(&line, &parser).unwrap_or_else(|err| panic!("Error for {line}: {err}"))
    });
    Ok(games.collect())
}

fn game_is_possible(game: &Game, dice_counts: &DiceCounts) -> bool {
    let draw_is_impossible = |draw: &Draw| {
        draw.iter().any(|(&color, &num_drawn)| {
            let &num_existing = dice_counts.get(color).unwrap_or(&0);
            num_drawn > num_existing
        })
    };

    !game.draws.iter().any(draw_is_impossible)
}

fn doit(data: &String) -> u64 {
    let mut dice_counts: DiceCounts = HashMap::new();
    dice_counts.insert("red", 12);
    dice_counts.insert("green", 13);
    dice_counts.insert("blue", 14);

    let games = parse_games(get_non_empty_lines(&data)).unwrap();

    games
        .iter()
        .filter(|&g| game_is_possible(g, &dice_counts))
        .fold(0u64, |acc, g| acc + g.id)
}

fn doit2(data: &String) -> u64 {
    let games = parse_games(get_non_empty_lines(&data)).unwrap();

    games.iter().fold(0u64, |acc, game| {
        let mut minimums: DiceCounts = HashMap::new();

        game.draws.iter().for_each(|draw| {
            draw.iter().for_each(|(&color, &count)| {
                let &old_min = minimums.get(color).unwrap_or(&0);
                if count > old_min {
                    minimums.insert(color, count);
                }
            })
        });

        let power = ["red", "green", "blue"].iter().fold(1u64, |acc, &color| {
            let &count = minimums.get(color).unwrap_or(&0);
            acc * count
        });

        acc + power
    })
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day2/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 8);

        let answer = doit2(data);
        assert_eq!(answer, 2286)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day2/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 2176);

        let answer = doit2(data);
        assert_eq!(answer, 63700)
    }
}
