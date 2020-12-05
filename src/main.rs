

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

mod day_01 {
    use ::std::mem::take;
    struct Data {
        // This is sorted on construction,
        // so functions can assume this is sorted for correctness.
        entries: Vec<u64>,
    }
    // This is only provided as infallible because
    // it is assumed the provided input will be well formed.
    // We panic otherwise.
    fn parse_input(input: &[u8]) -> Data {
        let mut entries = Vec::new();
        let mut current = Vec::new();
        let mut cursor = 0;

        loop {
            match input[cursor..] {
                [b'\n', ..] => {
                    cursor += 1;
                    entries.push(String::from_utf8(take(&mut current)).unwrap().parse().unwrap())
                },
                [b'\r', b'\n', ..] => {
                    cursor += 2;
                    entries.push(String::from_utf8(take(&mut current)).unwrap().parse().unwrap())
                },
                [x, ..] if x >= b'0' && x <= b'9' => {
                    cursor += 1;
                    current.push(x)
                }
                [x, ..] => panic!("Invalid input: {}, at position {}", x, cursor),
                [] => break,
            }
        }

        entries.sort();

        Data { entries }
    }
    // Infallible for the same reason as above.
    fn find_pair(data: &Data) -> (usize, usize) {
        for (idx, x) in data.entries.iter().enumerate() {
            match data.entries.binary_search(&(2020 - x)) {
                Ok(sdx) => return (idx, sdx),
                Err(_) => (),
            }
        }
        unreachable!("there should always be at least one match")
    }
    // Infallible for the same reason as above.
    pub fn solve(input: &[u8]) -> u64 {
        let data = parse_input(input);
        let (a, b) = find_pair(&data);
        let solution = data.entries[a] * data.entries[b];
        solution
    }
}

fn main() {
    println!("Day 1: {}", day_01::solve(&input::fetch(1).unwrap()));
}
