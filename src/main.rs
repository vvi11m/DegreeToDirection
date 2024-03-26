use rand::Rng;

fn deg_to_cardinal(deg: u16) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
    let index = if deg < 360 {
        (deg + 23) / 45
    } else {
        panic!("Angle {} is out of range. Must be between 0 and 359", deg);
    } as usize;
    println!("{}", (deg + 23) / 45);
    directions[index]
}

fn main() {
    let deg: u16 = rand::thread_rng().gen_range(0..400);
    let s: &str = deg_to_cardinal(deg);
    println!("{} {}", deg, s);
}