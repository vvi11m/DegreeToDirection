use rand::Rng;

fn deg_to_cardinal_acc(deg: &f64) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
    // goes to next on tie, see table below
    if *deg > 360.0 {
        panic!("Angle {} is out of range. Must be  0 and 360", deg);
    }
    let index = ((deg + 22.5) / 45.0) as usize;
    directions[index]
}

fn deg_to_cardinal_fast(deg: &u16) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
    let index = if *deg < 360 {
        (deg + 23) / 45
    } else {
        panic!("Angle {} is out of range. Must be between 0 and 359", deg);
    } as usize;
    directions[index]
}

fn main() {
    let deg_1: f64 = rand::thread_rng().gen_range(0.0..360.0);
    let deg_2: u16 = rand::thread_rng().gen_range(0..360);
    let r_1: &str = deg_to_cardinal_acc(&deg_1);
    let r_2: &str = deg_to_cardinal_fast(&deg_2);
    println!("accurate: {} {}", deg_1, r_1);
    println!("fast: {} {}", deg_2, r_2);
}

// 0/360 N
// 22.5 NE
// 45 NE
// 67.5 E
// 90 E
// 112.5 SE
// 135 SE
// 157.5 S
// 180 S
// 202.5 SW
// 225 SW
// 247.5 W
// 270 W
// 292.5 NW
// 315 NW
// 337.5 N
