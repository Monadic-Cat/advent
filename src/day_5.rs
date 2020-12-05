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
    let row = row.iter().enumerate().fold(0u8, |a, (idx, x)| {
        match x {
            b'B' => a | ROW_MAX >> idx,
            b'F' => a,
            _ => unreachable!()
        }
    });
    let column = &pass[7..];
    let column = column.iter().enumerate().fold(0u8, |a, (idx, x)| {
        match x {
            b'R' => a | COLUMN_MAX >> idx,
            b'L' => a,
            _ => unreachable!()
        }
    });
    BoardingPass {
        seat_id: (row as u16) * 8 + (column as u16),
        row, column,
    }
}
fn parse_input(input: &[u8]) -> Vec<BoardingPass> {
    ::core::str::from_utf8(input).unwrap().lines().map(|x| parse_pass(x.as_bytes())).collect()
}

pub fn solve(input: &[u8]) -> u16 {
    let passes = parse_input(input);
    passes.iter().map(|x| x.seat_id).max().unwrap()
}
