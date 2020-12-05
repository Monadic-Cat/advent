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
macro_rules! some_or_continue {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    }
}
fn find_pair(total: u64, data: &Data) -> Option<(usize, usize)> {
    for (idx, x) in data.entries.iter().enumerate() {
        match data.entries.binary_search(&some_or_continue!(total.checked_sub(*x))) {
            Ok(sdx) => return Some((idx, sdx)),
            Err(_) => (),
        }
    }
    None
}
fn find_triple(total: u64, data: &Data) -> Option<(usize, usize, usize)> {
    for (idx, x) in data.entries.iter().enumerate() {
        match find_pair(some_or_continue!(total.checked_sub(*x)), data) {
            Some((b, c)) => return Some((idx, b, c)),
            None => ()
        };
    }
    None
}
// Infallible for the same reason as above.
pub fn solve(input: &[u8]) -> (u64, u64) {
    let data = parse_input(input);
    let (a, b) = find_pair(2020, &data).expect("this is one of the required solutions");
    let part_one_solution = data.entries[a] * data.entries[b];
    let (a, b, c) = find_triple(2020, &data).expect("this is one of the required solutions");
    let part_two_solution = data.entries[a] * data.entries[b] * data.entries[c];
    (part_one_solution, part_two_solution)
}
