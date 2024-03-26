use rand::Rng;

fn deg_to_cardinal(deg: &u16) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
    if *deg > 360 {
        panic!("Angle {} is out of range. Must be  0 and 360", deg);
    }
    let index = ((deg + 23) / 45) as usize;
    directions[index]
    
}

fn main() {
    let deg: u16 = rand::thread_rng().gen_range(0..500);
    let s: &str = deg_to_cardinal(&deg);
    println!("{} {}", deg, s);
}