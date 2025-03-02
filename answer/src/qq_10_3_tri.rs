use std::{collections::HashSet, sync::{Arc, Mutex}, thread};

/*
已知钝角三角形ABC三边长均为正整数，并且三边长是互质的。
A到BC的距离和BC长度比例是10:3。
已知这个三角形不和三边长为6,25,29的三角形相似。
求BC的其中一个可能值。
*/

fn get_range(cpu_level: usize) -> u128 {
    if cpu_level == 0 {
        u128::MAX
    } else {
        ((u128::MAX >> 1) + 1) >> (cpu_level - 1)
    }
}

pub fn main() {
	let primer = Arc::new(Mutex::new(Primer::new()));
    let range = get_range(4);
    for t in 0..16 {
		let shared_primer = Arc::clone(&primer);
        thread::spawn(move || {
            let mut primer = shared_primer.lock().unwrap();
            for a in 2.max(t * range)..(t + 1) * range {
                if primer.is_not_prim(a) {
                    continue;
                }
                for b in a + 1..u128::MAX {
                    if primer.is_not_prim(b) {
                        continue;
                    }
                    for c in b + 1..a + b {
                        if primer.is_not_prim(c) {
                            continue;
                        }
                        if get_dis(a, b, c) == a / 3 {
                            println!("{a},{b},{c}");
                        }
                    }
                }
            }
			println!("{t} is end!");
        })
        .join()
        .unwrap();
    }
}

struct Primer {
    set: HashSet<u128>,
    calced_num: u128,
}
impl Primer {
    pub fn new() -> Primer {
        Primer {
            set: HashSet::new(),
            calced_num: 1,
        }
    }
    fn is_not_prim_impl(&mut self, n: u128) -> bool {
        for i in 2..n {
            if n % i == 0 {
                return true;
            }
        }
        self.set.insert(n);
        false
    }
    pub fn is_not_prim(&mut self, n: u128) -> bool {
        if n <= self.calced_num {
            return !self.set.contains(&n);
        }
        for i in self.calced_num + 1..n {
            self.is_not_prim_impl(i);
        }
        self.calced_num = n;
        self.is_not_prim_impl(n)
    }
}

fn get_dis(a: u128, b: u128, c: u128) -> u128 {
    let t = (b / 10) * (b / 2) - (c / 10) * (c / 2) + (a / 10) * (a / 2);
    return t / a;
}
