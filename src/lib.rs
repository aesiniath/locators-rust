use base62;

// a function which converts a number to base 16 but then uses the English16
// character set to represent it.
pub fn to_english16(n: u32) -> String {
    let mut result = String::new();
    let mut num = n;

    if n == 0 {
        return "0".to_owned();
    }

    while num > 0 {
        let digit = num % 16;
        let char = match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => 'C',
            4 => '4',
            5 => 'F',
            6 => 'H',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'R',
            14 => 'X',
            15 => 'Y',
            _ => unreachable!(),
        };
        result.push(char);
        num /= 16;
    }

    result
        .chars()
        .rev()
        .collect()
}

// convert from the english 16 alphabet to base 10 decimal
pub fn from_english16(s: &str) -> Option<u32> {
    let mut result = 0;

    if s.len() > "YYYYYYYY".len() {
        return None;
    }

    for c in s.chars() {
        let value = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            'C' => 3,
            '4' => 4,
            'F' => 5,
            'H' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'K' => 10,
            'L' => 11,
            'M' => 12,
            'R' => 13,
            'X' => 14,
            'Y' => 15,
            _ => return None,
        };
        result = result * 16 + value;
    }

    Some(result)
}

pub fn to_english16a(width: usize, n: i32) -> String {
    // Implement your toEnglish16a function here
    unimplemented!()
}

pub fn hash_string_to_english16a(width: usize, s: &str) -> String {
    // Implement your hashStringToEnglish16a function here
    unimplemented!()
}

pub fn pad_with_zeros(width: usize, s: &str) -> String {
    if s.len() >= width {
        s.to_owned()
    } else {
        let mut padded = String::with_capacity(width);
        for _ in 0..(width - s.len()) {
            padded.push('0');
        }
        padded.push_str(s);
        padded
    }
}

// convert a number to base62
pub fn to_base62(n: u64) -> String {
    // use the encoder from the base62 crate
    base62::encode(n)
}

pub fn from_base62(s: &str) -> Option<u64> {
    // use the decoder from the base62 crate
    let result = base62::decode(s);

    // truncate the result to u64
    match result {
        Ok(x) => Some(x as u64),
        Err(_) => None,
    }
}

pub fn hash_string_to_base62(width: usize, s: &str) -> String {
    // Implement your hashStringToBase62 function here
    unimplemented!()
}

pub fn to_latin25(n: u32) -> String {
    let mut result = String::new();
    let mut num = n;

    if n == 0 {
        return "0".to_owned();
    }

    while num > 0 {
        let digit = num % 25;
        let char = match digit {
            0 => '0',
            1 => '1',
            2 => '3',
            3 => '4',
            4 => '7',
            5 => '8',
            6 => '9',
            7 => 'A',
            8 => 'C',
            9 => 'E',
            10 => 'G',
            11 => 'H',
            12 => 'J',
            13 => 'K',
            14 => 'L',
            15 => 'M',
            16 => 'N',
            17 => 'P',
            18 => 'S',
            19 => 'T',
            20 => 'V',
            21 => 'W',
            22 => 'X',
            23 => 'Y',
            24 => 'Z',
            _ => unreachable!(),
        };
        result.push(char);
        num /= 25;
    }

    result
        .chars()
        .rev()
        .collect()
}

fn greater_than(s1: &str, s2: &str) -> bool {
    let l1 = s1.len();
    let l2 = s2.len();

    if l1 > l2 {
        return true;
    }

    if l1 == l2 {
        return s1 > s2;
    }

    false
}

pub fn from_latin25(s: &str) -> Option<u32> {
    let mut result = 0;

    if greater_than(s, "LygHa16AHYG") {
        return None;
    }

    for c in s.chars() {
        let value = match c {
            '0' => 0,
            '1' => 1,
            '3' => 2,
            '4' => 3,
            '7' => 4,
            '8' => 5,
            '9' => 6,
            'A' => 7,
            'C' => 8,
            'E' => 9,
            'G' => 10,
            'H' => 11,
            'J' => 12,
            'K' => 13,
            'L' => 14,
            'M' => 15,
            'N' => 16,
            'P' => 17,
            'S' => 18,
            'T' => 19,
            'V' => 20,
            'W' => 21,
            'X' => 22,
            'Y' => 23,
            'Z' => 24,
            _ => return None,
        };
        result = result * 25 + value;
    }

    Some(result)
}

pub fn hash_string_to_latin25(width: usize, s: &str) -> String {
    // Implement your hashStringToLatin25 function here
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    use std::panic;

    #[test]
    fn check_int_to_base62() {
        assert_eq!(to_base62(1), "1");
        assert_eq!(to_base62(61), "z");
        assert_eq!(to_base62(62), "10");
        assert_eq!(to_base62(63), "11");
        assert_eq!(to_base62(u64::MAX), "LygHa16AHYF");
    }

    #[test]
    fn check_base62_to_int() {
        assert_eq!(from_base62("1"), Some(1));
        assert_eq!(from_base62("z"), Some(61));
        assert_eq!(from_base62("10"), Some(62));
        assert_eq!(from_base62("11"), Some(63));
        assert_eq!(from_base62("LygHa16AHYF"), Some(u64::MAX));

        assert_eq!(from_base62("!"), None);
        assert_eq!(from_base62(" "), None);
        assert_eq!(from_base62("1 2"), None);
    }

    #[test]
    fn check_known_english16() {
        assert_eq!(to_english16(0x000001), "1");
        assert_eq!(to_english16(0x000010), "10");
        assert_eq!(to_english16(0x000100), "100");
        assert_eq!(to_english16(u32::MIN), "0");
        assert_eq!(to_english16(u32::MAX), "YYYYYYYY");

        assert_eq!(from_english16("1"), Some(1));
        assert_eq!(from_english16("10"), Some(16));
        assert_eq!(from_english16("100"), Some(256));
        assert_eq!(from_english16("0"), Some(0));
        assert_eq!(from_english16("YYYYYYYY"), Some(u32::MAX));
        assert_eq!(from_english16("100000000"), None);
    }

    #[test]
    fn check_round_trip_english16() {
        fn prop_english16(n: u32) -> bool {
            match from_english16(&to_english16(n)) {
                Some(decoded) => n == decoded,
                None => false,
            }
        }
        quickcheck(prop_english16 as fn(u32) -> bool);
    }

    #[test]
    fn check_known_english16a() {
        assert_eq!(to_english16a(6, 0x111111), "12C4FH");
        assert_eq!(to_english16a(6, 0x777777), "789KLM");
        assert_eq!(to_english16a(6, 0xCCCCCC), "MRXY01");
    }

    #[test]
    fn test_problematic_edge_cases() {
        assert_eq!(to_english16a(6, 0x0), "012C4F");
        assert_eq!(hash_string_to_english16a(6, "perf_data"), "FHL417");
        assert_eq!(
            hash_string_to_english16a(6, "perf_data/bletchley"),
            "K48F01"
        );
    }

    #[test]
    fn check_left_padding() {
        assert_eq!(pad_with_zeros(5, "1"), "00001");
        assert_eq!(pad_with_zeros(5, "123456"), "123456");
        assert_eq!(pad_with_zeros(10, &to_base62(u64::MAX)), "LygHa16AHYF");
        assert_eq!(pad_with_zeros(11, &to_base62(u64::MAX)), "LygHa16AHYF");
        assert_eq!(pad_with_zeros(12, &to_base62(u64::MAX)), "0LygHa16AHYF");
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(to_english16a(1, -1), "1");
    }

    #[test]
    fn check_bounds() {
        assert_eq!(greater_than("0", "Hello"), false);
        assert_eq!(greater_than("1", "Hello"), false);
        assert_eq!(greater_than("H", "Hello"), false);
        assert_eq!(greater_than("Hello", "Hello"), false);
        assert_eq!(greater_than("Hellp", "Hello"), true);
        assert_eq!(greater_than("A00000", "Hello"), true);
        assert_eq!(greater_than("I", "Hello"), false);
    }

    #[test]
    fn check_known_latin25() {
        assert_eq!(to_latin25(0), "0");
        assert_eq!(to_latin25(1), "1");
        assert_eq!(to_latin25(24), "Z");
        assert_eq!(to_latin25(25), "10");
        assert_eq!(to_latin25(3662109375), "M000000");

        assert_eq!(from_latin25("0"), Some(0));
        assert_eq!(from_latin25("1"), Some(1));
        assert_eq!(from_latin25("2"), None);
        assert_eq!(from_latin25("3"), Some(2));
        assert_eq!(from_latin25("Z"), Some(24));
        assert_eq!(from_latin25("10"), Some(25));

        // assert_eq!(from_latin25("M000000"), Some(3662109375));
    }

    #[test]
    fn check_round_trip_latin25() {
        fn prop_latin25(n: u32) -> bool {
            match from_latin25(&to_latin25(n)) {
                Some(decoded) => n == decoded,
                None => false,
            }
        }
        quickcheck(prop_latin25 as fn(u32) -> bool);
    }

    #[test]
    fn test_hash_latin25() {
        assert_eq!(
            hash_string_to_latin25(
                5,
                "You'll get used to it. Or, you'll have a psychotic episode"
            ),
            "XSAV1"
        );
    }

    #[test]
    fn test_width_guards_english16a() {
        let result = panic::catch_unwind(|| to_english16a(17, 1));
        assert!(result.is_err());
    }

    #[test]
    fn test_width_guards_hashing() {
        assert_eq!(hash_string_to_latin25(17, "a").len(), 17);
        let result = panic::catch_unwind(|| hash_string_to_latin25(18, "a"));
        assert!(result.is_err());
    }
}
