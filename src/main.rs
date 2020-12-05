mod day_1;
mod day_2;

mod input {
    use ::std::io::Result;
    // const BASE_URL: &str = "https://adventofcode.com/2020";
    
    // async fn fetch_input(client: &::reqwest::Client, day: u8) -> Result<Bytes> {
    //     Ok(client.get(&format!("{}/day/{}/input", BASE_URL, day)).send().await?.bytes().await?)
    // }
    pub fn fetch(day: u8) -> Result<Vec<u8>> {
        ::std::fs::read(format!("{}/input/day_{}", env!("CARGO_MANIFEST_DIR"), day))
    }
}

fn main() {
    println!("Day 1: {:?}", day_1::solve(&input::fetch(1).unwrap()));
    println!("Day 2: {:?}", day_2::solve(&input::fetch(2).unwrap()));
}
