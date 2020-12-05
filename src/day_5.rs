const ROW_MAX: u8 = 1 << 6;
const COLUMN_MAX: u8 = 1 << 2;

#[derive(Debug)]
struct BoardingPass {
    row: u8,
    column: u8,
    seat_id: u16,
}
fn parse_pass(pass: &[u8]) -> BoardingPass {
    let row = &pass[..7];
    let row = row.iter().enumerate().fold(0u8, |a, (idx, x)| match x {
        b'B' => a | ROW_MAX >> idx,
        b'F' => a,
        _ => unreachable!(),
    });
    let column = &pass[7..];
    let column = column.iter().enumerate().fold(0u8, |a, (idx, x)| match x {
        b'R' => a | COLUMN_MAX >> idx,
        b'L' => a,
        _ => unreachable!(),
    });
    BoardingPass {
        seat_id: (row as u16) * 8 + (column as u16),
        row,
        column,
    }
}
fn parse_input(input: &[u8]) -> Vec<BoardingPass> {
    ::core::str::from_utf8(input)
        .unwrap()
        .lines()
        .map(|x| parse_pass(x.as_bytes()))
        .collect()
}

/// Find unused seat ID. Also sorts the list of passes.
fn find_empty(passes: &mut [BoardingPass]) -> u16 {
    passes.sort_unstable_by_key(|x| x.seat_id);
    let empty_seat = passes
        .windows(2)
        .filter_map(|x| match x {
            [BoardingPass { seat_id: a, .. }, BoardingPass { seat_id: b, .. }] if b - a != 1 => {
                Some(b - 1)
            }
            [_, _] => None,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()[0];
    empty_seat
}

pub fn solve(input: &[u8]) -> (u16, u16) {
    let mut passes = parse_input(input);
    let part_one = passes.iter().map(|x| x.seat_id).max().unwrap();
    let part_two = find_empty(&mut passes);
    (part_one, part_two)
}
