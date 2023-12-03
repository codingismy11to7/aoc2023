use std::fmt::Formatter;
use std::{fmt, fs};

pub struct DataLine<'a> {
    pub line: &'a str,
    pub line_number: usize,
}

impl fmt::Display for DataLine<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Line {}: ", self.line_number))?;
        f.write_str(self.line)?;
        Ok(())
    }
}

pub fn get_non_empty_lines<'a>(data: &'a String) -> impl Iterator<Item = DataLine<'a>> {
    let lines = data.split('\n').filter(|x| !x.is_empty());
    lines.zip(0..).map(|(line, num)| DataLine {
        line,
        line_number: num + 1,
    })
}

pub fn read_file_panic(fname: &str) -> String {
    fs::read_to_string(fname).expect("couldn't read file")
}
