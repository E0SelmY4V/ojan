use crate::input;

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
