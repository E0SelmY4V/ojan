use ojan_luogu::*;

fn power(n: u16, m: usize) -> u128 {
    (n as u128) << m
}

pub fn main() {
    let mut demander = input::demand();
    let n: usize = demander.get();
    let m: usize = demander.get();
    let sum: u128 = (0..n)
        .map(|_| {
            let line: Vec<u16> = vec![0]
                .into_iter()
                .chain(demander.get_many(m))
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
