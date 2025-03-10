macro_rules! use_as_now {
    ($mod:ident) => {
        mod $mod;
        use $mod as now;
    };
}

use_as_now!(p1480);

pub fn main() {
    now::main();
}

