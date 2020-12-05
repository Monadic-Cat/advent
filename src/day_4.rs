use ::core::mem;

#[derive(Default, Debug)]
struct Passport<'a> {
    birth_year: Option<&'a [u8]>,
    issue_year: Option<&'a [u8]>,
    expiration_year: Option<&'a [u8]>,
    height: Option<&'a [u8]>,
    hair_color: Option<&'a [u8]>,
    eye_color: Option<&'a [u8]>,
    passport_id: Option<&'a [u8]>,
    country_id: Option<&'a [u8]>,
}
impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        match (
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
            // Notice how we just ignore this field.
            // Mwahahahahahahahaaaa!
            self.country_id,
        ) {
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), _) => true,
            _ => false,
        }
    }
    fn is_valid_part_two(&self) -> bool {
        // The validity requirements from part 1 still hold,
        // but there are some additional requirements.
        // So, let's check part 1's, then do part 2's.
        if !self.is_valid() {
            false
        } else {
            parse_radix(self.birth_year.unwrap(), 10).and_then(|x| {
                match x {
                    x if (1920 ..= 2002).contains(&x) => Ok(x),
                    _ => Err(()),
                }
            }).is_ok() &&
                parse_radix(self.issue_year.unwrap(), 10).and_then(|x| match x {
                    x if (2010 ..= 2020).contains(&x) => Ok(x),
                    _ => Err(())
                }).is_ok() &&
                parse_radix(self.expiration_year.unwrap(), 10).and_then(|x| match x {
                    x if (2020 ..= 2030).contains(&x) => Ok(x),
                    _ => Err(()),
                }).is_ok() &&
                height_is_valid(self.height.unwrap()) &&
                hair_color_is_valid(self.hair_color.unwrap()) &&
                eye_color_is_valid(self.eye_color.unwrap()) &&
                passport_id_is_valid(self.passport_id.unwrap())
        }
    }
    /// Merge a field value into this passport.
    fn merge_field(&mut self, field: Field<'a>) {
        match field {
            Field::BirthYear(x) => self.birth_year = Some(x),
            Field::IssueYear(x) => self.issue_year = Some(x),
            Field::ExpirationYear(x) => self.expiration_year = Some(x),
            Field::Height(x) => self.height = Some(x),
            Field::HairColor(x) => self.hair_color = Some(x),
            Field::EyeColor(x) => self.eye_color = Some(x),
            Field::PassportID(x) => self.passport_id = Some(x),
            Field::CountryID(x) => self.country_id = Some(x),
        }
    }
}

enum Field<'a> {
    BirthYear(&'a [u8]),
    IssueYear(&'a [u8]),
    ExpirationYear(&'a [u8]),
    Height(&'a [u8]),
    HairColor(&'a [u8]),
    EyeColor(&'a [u8]),
    PassportID(&'a [u8]),
    CountryID(&'a [u8]),
}

fn take_until<F: Fn(u8) -> bool>(input: &[u8], predicate: F) -> (&[u8], &[u8]) {
    let mut cursor = 0;
    loop {
        match input[cursor..] {
            [x, ..] if !predicate(x) => cursor += 1,
            _ => break,
        }
    }
    (&input[cursor..], &input[..cursor])
}

/// Consumes next field, and returns a slice to its value.
/// Does not consume terminating space or newline.
/// Returns None if exists leading space or newline.
fn parse_kv_pair(input: &[u8]) -> (&[u8], Option<Field<'_>>) {
    match input {
        [b'\n', rest @ ..] => return (rest, None),
        [b' ', rest @ ..] => return (rest, None),
        _ => (),
    };
    let (rest, key) = take_until(input, |x| x == b':');
    let rest = &rest[1..]; // Throw away the b':' between the key and value.
    let (mut rest, value) = take_until(rest, |x| x == b' ' || x == b'\n');
    // Throw away terminating whitespace, if present.
    // This is in question because we *will* hit EOF.
    match rest {
        [x, ..] if (*x == b' ') || (*x == b'\n') => rest = &rest[1..],
        _ => (),
    };
    let field = match key {
        b"byr" => Field::BirthYear(value),
        b"iyr" => Field::IssueYear(value),
        b"eyr" => Field::ExpirationYear(value),
        b"hgt" => Field::Height(value),
        b"hcl" => Field::HairColor(value),
        b"ecl" => Field::EyeColor(value),
        b"pid" => Field::PassportID(value),
        b"cid" => Field::CountryID(value),
        x => panic!("unknown field type: {}", ::core::str::from_utf8(x)
                    .expect("failed utf8 conversion of unknown field type"))
    };
    (rest, Some(field))
}

fn parse_radix(x: &[u8], radix: u32) -> Result<u64, ()> {
    let text = ::core::str::from_utf8(x).unwrap();
    u64::from_str_radix(text, radix).map_err(|_| ())
}

fn height_is_valid(height: &[u8]) -> bool {
    match height {
        [amt @ .., b'c', b'm'] | [amt @ .., b'i', b'n'] => {
            parse_radix(amt, 10).and_then(|x| match height[height.len() - 2..] {
                [b'c', b'm'] if (150 ..= 193).contains(&x) => Ok(x),
                [b'i', b'n'] if (59 ..= 76).contains(&x) => Ok(x),
                _ => Err(()),
            }).is_ok()
        }
        _ => false,
    }
}

fn hair_color_is_valid(color: &[u8]) -> bool {
    match color {
        [b'#', rest @ ..] => {
            for (idx, b) in rest.iter().enumerate() {
                if !((b'0' ..= b'9').contains(b) || (b'a' ..= b'f').contains(b)) {
                    return false
                }
                if idx > 5 {
                    return false
                }
            }
            true
        }
        _ => false,
    }
}

fn eye_color_is_valid(color: &[u8]) -> bool {
    match color {
        b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth" => true,
        _ => false,
    }
}

fn passport_id_is_valid(id: &[u8]) -> bool {
    id.len() == 9 && parse_radix(id, 10).is_ok()
}

fn parse_input(mut input: &[u8]) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut current_passport = Passport::default();
    while input.len() > 0 {
        let (rest, field) = parse_kv_pair(input);
        if let Some(field) = field {
            current_passport.merge_field(field);
        } else {
            passports.push(mem::take(&mut current_passport));
        }
        input = rest;
    }

    passports
}
pub fn solve(input: &[u8]) -> (usize, usize) {
    let passports = parse_input(input);
    (passports.iter().filter(|x| x.is_valid()).count(),
     passports.iter().filter(|x| x.is_valid_part_two()).count())
}
