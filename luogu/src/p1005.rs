mod input {
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
}

fn power(n: u16, m: usize) -> u128 {
    (n as u128) << m
}

pub fn main() {
    let mut iner = input::new();
    let mut line_iner = iner.line();
    let n: usize = line_iner.parse();
    let m: usize = line_iner.parse();
    let sum: u128 = (0..n)
        .map(|_| {
            let line: Vec<u16> = vec![0]
                .into_iter()
                .chain(iner.line().parse_to_iter())
                .chain(vec![0].into_iter())
                .collect();
            let mut dp = [[0; 90]; 90];
            for width in (0..m).rev() {
                let pos = m - width;
                for from in 1..(pos + 2) {
                    dp[from][width] = u128::max(
                        dp[from - 1][width + 1] + power(line[from - 1], pos),
                        dp[from][width + 1] + power(line[from + width], pos),
                    );
                }
            }
            dp.into_iter().map(|a| a[0]).max().unwrap()
        })
        .sum();
    print!("{sum}");
}
