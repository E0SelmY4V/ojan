use ojan_luogu::*;

fn get_stage(sum: u16, stage_line: u16) -> (u16, u16) {
    if sum > stage_line {
        (stage_line, sum - stage_line)
    } else {
        (sum, 0)
    }
}

pub fn main() {
    let mut iner = input::new();
    let sum: u16 = iner.line().parse();
    let (sum, stage3) = get_stage(sum, 400);
    let (stage1, stage2) = get_stage(sum, 150);
    print!(
        "{:.1}",
        (stage3 as f64 * 0.5663) + (stage2 as f64 * 0.4663) + (stage1 as f64 * 0.4463)
    );
}
