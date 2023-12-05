use std::fmt::Formatter;
use std::{fmt, fs};

pub struct DataLine<'a> {
    pub line: &'a str,
    pub line_number: usize,
}

impl fmt::Display for DataLine<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Line {}: ", self.line_number + 1))?;
        f.write_str(self.line)?;
        Ok(())
    }
}

pub fn get_non_empty_lines(data: &str) -> impl Iterator<Item = DataLine> {
    data.split('\n')
        .filter(|x| !x.is_empty())
        .zip(0..)
        .map(|(line, line_number)| DataLine { line, line_number })
}

pub fn read_file_panic(fname: &str) -> String {
    fs::read_to_string(fname).expect("couldn't read file")
}
