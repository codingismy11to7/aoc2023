use regex::Regex;

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

fn parse_data(data: &str) -> Vec<Record> {
    let num_re = Regex::new(r"\d+").unwrap();
    let (times, distances) = data.split_once('\n').unwrap();
    let i = |s: &str| -> u64 { s.parse().unwrap() };

    num_re
        .find_iter(times)
        .zip(num_re.find_iter(distances))
        .map(|(t, d)| Record {
            time: i(t.as_str()),
            distance: i(d.as_str()),
        })
        .collect()
}

fn number_of_ways_to_win(prev_rec: &Record) -> u64 {
    let num_skipped = (0..prev_rec.time)
        .find(|&i| (i * (prev_rec.time - i)) > prev_rec.distance)
        .unwrap();
    (prev_rec.time + 1) - (num_skipped * 2)
}

fn doit(data: &str) -> u64 {
    parse_data(data).iter().map(number_of_ways_to_win).product()
}

fn doit2(data: &str) -> u64 {
    let num_re = Regex::new(r"\d+").unwrap();
    let (time, distance) = data.split_once('\n').unwrap();
    let i = |s: String| -> u64 { s.parse().unwrap() };

    let time = i(num_re.find_iter(time).map(|m| m.as_str()).collect());
    let distance = i(num_re.find_iter(distance).map(|m| m.as_str()).collect());

    number_of_ways_to_win(&Record { time, distance })
}

#[cfg(test)]
mod tests {
    use crate::util::read_file_panic;

    use super::*;

    #[test]
    fn t() {
        let data = &read_file_panic("./data/day6/test.txt");
        let answer = doit(data);
        assert_eq!(answer, 288);

        let answer = doit2(data);
        assert_eq!(answer, 71503)
    }

    #[test]
    fn d() {
        let data = &read_file_panic("./data/day6/data.txt");
        let answer = doit(data);
        assert_eq!(answer, 2344708);

        let answer = doit2(data);
        assert_eq!(answer, 30125202)
    }
}
