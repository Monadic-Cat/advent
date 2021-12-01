#[derive(Copy, Clone)]
enum Space {
    Open,
    Tree,
}
struct Map {
    rows: Vec<Vec<Space>>,
}
struct Point {
    x: usize,
    y: usize,
}
struct Slope {
    right: usize,
    down: usize,
}
impl Map {
    fn get(&self, p: &Point) -> Space {
        let row = &self.rows[p.y];
        row[p.x % row.len()]
    }
}

fn parse_input(input: &[u8]) -> Map {
    let rows: Vec<Vec<Space>> = ::core::str::from_utf8(input)
        .unwrap()
        .lines()
        .map(|x| x.bytes().map(|x| match x {
            b'.' => Space::Open,
            b'#' => Space::Tree,
            _ => unreachable!("invalid space"),
        }).collect())
        .collect();
    Map { rows }
}

/// Count the number of trees we encounter
fn sled(start: Point, slope: Slope, map: &Map) -> u64 {
    let mut current = start;
    let mut tree_count = 0;
    while current.y < map.rows.len() {
        match map.get(&current) {
            Space::Open => (),
            Space::Tree => tree_count += 1,
        }
        current.x += slope.right;
        current.y += slope.down;
    }
    tree_count
}

macro_rules! sled_many {
    (point: ($px:expr , $py:expr), map: $map:expr => { $(($right:expr , $down:expr)),+ $(,)? }) => {
        vec![$(
            sled(Point { x: $px, y: $py },
                 Slope { right: $right, down: $down },
                 $map)
        ),+]
    }
}

pub fn solve(input: &[u8]) -> (u64, u64) {
    let map = parse_input(input);
    let part_one = sled(Point { x: 0, y: 0 }, Slope { right: 3, down: 1 }, &map);
    let part_two = sled_many! { point: (0, 0),
                                map: &map => {
                                    (1, 1),
                                    (3, 1),
                                    (5, 1),
                                    (7, 1),
                                    (1, 2),
                                }
    }.iter().product();
    (part_one, part_two)
}
