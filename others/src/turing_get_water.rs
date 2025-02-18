fn get_water(terrain: &[usize]) -> usize {
    let mut left = [0; 17];
    let mut right = [0; 17];
    for addr in 0..16 {
        if terrain[addr] > left[addr] {
            left[addr + 1] = terrain[addr];
        } else {
            left[addr + 1] = left[addr];
        }
    }
    for addr in (0..16).rev() {
        if terrain[addr] > right[addr + 1] {
            right[addr] = terrain[addr];
        } else {
            right[addr] = right[addr + 1];
        }
    }
    let mut water = 0;
    for addr in 0..16 {
        water += left[addr + 1].min(right[addr]) - terrain[addr];
    }
    water
}

pub fn main() {
    let terrain = [4, 6, 1, 4, 6, 5, 1, 4, 1, 2, 6, 5, 6, 1, 4, 2];
    println!("{}", get_water(&terrain));
}
