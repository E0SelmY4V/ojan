use ojan_luogu::*;

type Num = u32;
struct Pack {
    num: Num,
    cost: Num,
}
impl Pack {
    pub fn cost(&self, n: Num) -> Num {
        (match n / self.num {
            d if d * self.num == n => d,
            d => d + 1,
        }) * self.cost
    }
}

pub fn main() {
    let mut iner = input::new();
    let n: Num = iner.line().parse();
    let min_cost = (0..3)
        .map(|_| iner.line().parse_to_iter().take(2).collect::<Vec<Num>>())
        .map(|n| Pack {
            num: n[0],
            cost: n[1],
        })
        .map(|p| p.cost(n))
        .min()
        .unwrap();
    print!("{min_cost}");
}
