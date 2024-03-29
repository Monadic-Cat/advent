use ::core::ops::RangeInclusive;
type PolicyT = (u8, RangeInclusive<u64>);
#[derive(Debug)]
struct Entry<'a> {
    policy: PolicyT,
    password: &'a [u8],
}
impl Entry<'_> {
    fn is_valid(&self) -> bool {
        self.policy.1.contains(&(self.password.iter().filter(|b| **b == self.policy.0).count() as u64))
    }
}
fn parse_input(input: &[u8]) -> Vec<Entry<'_>> {
    #[derive(Debug)]
    enum State {
        Policy(PolicyMode),
        // If we're in password mode, we already have a policy.
        Password { policy: PolicyT },
        Nothing,
    }
    #[derive(Debug)]
    enum PolicyMode {
        Range(RangeMode, (u64, u64)),
        Letter { range: RangeInclusive<u64> },
    }
    #[derive(Debug)]
    enum RangeMode {
        First, Second,
    }
    use State::*;
    use PolicyMode::*;
    use RangeMode::*;
    let mut entries = Vec::new();
    let mut state = Policy(Range(First, (0, 0)));
    let mut mark = 0;
    let mut cursor = 0;

    loop {
        match state {
            Policy(Range(ref mut p, ref mut tup)) => match input[cursor..] {
                // A dash ends a range bound.
                [b'-', ..] => {
                    tup.0 = ::core::str::from_utf8(&input[mark..cursor]).unwrap().parse().unwrap();
                    *p = Second;
                    cursor += 1;
                    mark = cursor;
                },
                // A space ends a range.
                [b' ', ..] => {
                    tup.1 = ::core::str::from_utf8(&input[mark..cursor]).unwrap().parse().unwrap();
                    cursor += 1;
                    mark = cursor;
                    // Transition to Letter mode.
                    state = Policy(Letter {
                        // These ranges are inclusive.
                        range: (tup.0 ..= tup.1),
                    });
                },
                // Consume a digit.
                [x, ..] if x >= b'0' && x <= b'9' => cursor += 1,
                [x, ..] => unreachable!("invalid input: {} at position {}", x, cursor),
                [] => unreachable!("the policy range will *never* be at the end of input"),
            },
            Policy(Letter { range }) => match input[cursor..] {
                [x, b':', b' ', ..] => {
                    state = Password { policy: (x, range) };
                    cursor += 3;
                    mark += 3;
                },
                [..] => unreachable!("invalid policy shape"),
            },
            Password { policy } => match input[cursor..] {
                // Terminate password.
                [b'\n', ..] => {
                    entries.push(Entry {
                        password: &input[mark..cursor],
                        policy,
                    });
                    state = Nothing;
                    cursor += 1;
                    mark = cursor;
                },
                // Terminate password.
                [b'\r', b'\n', ..] => todo!("CRLF style line breaks"),
                // Consume letter of password.
                [_, ..] => {
                    // We moved the policy out, put it back.
                    state = Password { policy };
                    cursor += 1
                },
                // End of input, so end iteration.
                [] => break,
            },
            Nothing => match input[cursor..] {
                [_, ..] => state = Policy(Range(First, (0, 0))),
                [] => break,
            }
        }
    }

    entries
}
pub fn solve(input: &[u8]) -> (u64, u64) {
    let entries = parse_input(input);
    let part_one = entries.iter().filter(|e| e.is_valid()).count() as u64;
    let part_two = entries.iter().filter(|e| {
        let (fdx, sdx) = (e.policy.1.start(), e.policy.1.end());
        // The given positions are 1-indexed.
        let (fdx, sdx) = (fdx - 1, sdx - 1);
        let letter = e.policy.0;
        let word = e.password;
        (word[usize::try_from(fdx).unwrap()] == letter) ^
            (word[usize::try_from(sdx).unwrap()] == letter)
    }).count() as u64;
    (part_one, part_two)
}
