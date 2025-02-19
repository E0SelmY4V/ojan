use std::{
    fmt::Debug,
    io::{self, Stdin},
    str::{FromStr, SplitWhitespace},
};
pub struct Iner {
    input: String,
    stdin: Stdin,
}
pub fn new() -> Iner {
    Iner {
        input: String::new(),
        stdin: io::stdin(),
    }
}
impl Iner {
    pub fn line(&mut self) -> LineIner {
        self.input.clear();
        self.stdin
            .read_line(&mut self.input)
            .expect("Can't read input!");
        let splited = self.input.split_whitespace();
        LineIner { splited }
    }
}
pub struct LineIner<'a> {
    pub splited: SplitWhitespace<'a>,
}
impl<'a> LineIner<'a> {
    pub fn parse<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.splited
            .next()
            .expect("No more input!")
            .parse()
            .expect("can't parse!")
    }
    pub fn parse_to_iter<T>(self) -> impl Iterator<Item = T> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.splited.map(|s| s.parse().expect("can't parse!"))
    }
}
