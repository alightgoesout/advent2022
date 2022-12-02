use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub trait Parse<T> {
    fn parse(self) -> T;
}

impl<I, T, U> Parse<Vec<U>> for I
where
    I: Iterator<Item = T>,
    T: ToString,
    U: FromStr,
    U::Err: Debug,
{
    fn parse(self) -> Vec<U> {
        self.map(|line| line.to_string().parse())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect()
    }
}

pub fn read_lines<R: Read>(reader: R) -> impl Iterator<Item = String> {
    let buf_reader = BufReader::new(reader);
    buf_reader
        .lines()
        .filter(Result::is_ok)
        .map(|line| line.unwrap().trim().to_string())
}

pub fn read_lines_from_file(name: &str) -> impl Iterator<Item = String> {
    read_lines(File::open(format!("src/input/{}", name)).unwrap())
}
