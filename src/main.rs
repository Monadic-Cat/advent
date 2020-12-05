mod day_1;
mod day_2;

mod input {
    use ::std::io::Result;
    pub fn fetch(day: u8) -> Result<Vec<u8>> {
        ::std::fs::read(format!("{}/input/day_{}", env!("CARGO_MANIFEST_DIR"), day))
    }
}

fn main() {
    println!("Day 1: {:?}", day_1::solve(&input::fetch(1).unwrap()));
    println!("Day 2: {:?}", day_2::solve(&input::fetch(2).unwrap()));
}
