use ojan::*;

pub fn main() {
    let mut iner = input::new();
    let mut line_iner = iner.line();
    let x: u64 = line_iner.parse();
    let n: u64 = line_iner.parse();
    let sum: u64 = ((x - 1)..(x + n - 1)).map(|i| (i % 7 < 5) as u64).sum();
    print!("{}", sum * 250);
}

/*

0 1 2 3 4 5 6 7
t t t t t f f t
a_n = n % 7 < 5

           d_n - (n%7<5)*250 = d_{n-1}
d_n - 250\sum_{i=1}^n(i%7<5) = d_{n-1} - 250\sum_{i=1}^{n-1}(i%7<5) = d_1 - 250
                         d_n = d_1 + 250\sum_{i=2}^n(i%7<5)

  d_(x-1+n) - d_(x-1)
=  250(\sum_{i=2}^(x-1+n)(i%7<5) - \sum_{i=2}^(x-1)(i%7<5))
=  250\sum_{i=x}^(x-1+n)(i%7<5)
*/
