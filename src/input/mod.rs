use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::marker::PhantomData;
use std::str::FromStr;

/*pub trait Parse {
    fn parse<T>(self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug;
}

impl<I, U> Parse for I
where
    I: Iterator<Item = U>,
    U: ToString,
{
    fn parse<T>(self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.map(|line| line.to_string().parse())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect()
    }
}*/

pub struct Parse<I, T>(I, PhantomData<T>);

impl<I, U, T> Iterator for Parse<I, T>
where
    I: Iterator<Item = U>,
    U: ToString,
    T: FromStr,
    T::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.to_string().parse().unwrap())
    }
}

pub trait ParseExt<I> {
    fn parse<T>(self) -> Parse<I, T>;
}

impl<I: Iterator> ParseExt<I> for I {
    fn parse<T>(self) -> Parse<I, T> {
        Parse(self, PhantomData::default())
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
