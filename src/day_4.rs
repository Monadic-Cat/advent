use ::core::mem::take;

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

fn parse_input(mut input: &[u8]) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut current_passport = Passport::default();
    while input.len() > 0 {
        let (rest, field) = parse_kv_pair(input);
        if let Some(field) = field {
            current_passport.merge_field(field);
        } else {
            passports.push(take(&mut current_passport));
        }
        input = rest;
    }

    passports
}
pub fn solve(input: &[u8]) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|x| x.is_valid()).count()
}
