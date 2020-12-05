mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

mod input {
    use ::std::io::Result;
    pub fn fetch(day: u8) -> Result<Vec<u8>> {
        ::std::fs::read(format!("{}/input/day_{}", env!("CARGO_MANIFEST_DIR"), day))
    }
}

fn main() {
    println!("Day 1: {:?}", day_1::solve(&input::fetch(1).unwrap()));
    println!("Day 2: {:?}", day_2::solve(&input::fetch(2).unwrap()));
    println!("Day 3: {:?}", day_3::solve(&input::fetch(3).unwrap()));
    println!("Day 4: {:?}", day_4::solve(&input::fetch(4).unwrap()));
    println!("Day 5: {:?}", day_5::solve(&input::fetch(5).unwrap()));
}
