//! A symbol set with twenty-five visually distinct characters.
//!
//! This alphabet's letters were chosen to maximize the contrast in glyph
//! shapes and to avoid ambiguities. So, for example, only one of `'2'` and
//! `'Z'` can be a letter of the alphabet (here we chose the latter for it
//! being more angular). Likewise only one of `'O'`, `'Q'`, and `'0'` is
//! available; having chosen `'0'` to represent zero the other candidates are
//! knocked out.
//!
//! These are not protected against similar pronounciations; if you need to
//! read your identifiers _aloud_ use [English16](crate::english16) instead.

use crate::greater_than;
use sha2::Digest;

/// Given a number, convert it to a string in the Latin25 base 25 symbol
/// alphabet.
///
/// This is useful for primary keys and object identifiers that you need to
/// scan for in log output, for example.
pub fn to_latin25(n: u64) -> String {
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

/// Given a number encoded in the Latin25 alphabet, convert it back to a base
/// 10 decimal.
pub fn from_latin25(s: &str) -> Option<u64> {
    let mut result = 0;

    if greater_than(s, "JEJ449Z0XWHALM") {
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

/// Take an arbitrary sequence of bytes, hash it with SHA256, then format as a
/// Latin25 string.
///
/// ```
/// hash_string_to_latin25 "You'll get used to it. Or, you'll have a psychotic episode"
/// ```
/// will result in `"E4AJ4G9E0T8Z8T"`.
pub fn hash_string_to_latin25(s: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();

    // Convert the first 8 bytes of the hash to a u64
    let (bytes, _) = result.split_at(std::mem::size_of::<u64>());
    let result = u64::from_le_bytes(
        bytes
            .try_into()
            .unwrap(),
    );

    to_latin25(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    #[test]
    fn check_known_latin25() {
        assert_eq!(to_latin25(0), "0");
        assert_eq!(to_latin25(1), "1");
        assert_eq!(to_latin25(24), "Z");
        assert_eq!(to_latin25(25), "10");
        assert_eq!(to_latin25(3662109375), "M000000");
        assert_eq!(to_latin25(u64::MAX), "JEJ449Z0XWHALM");

        assert_eq!(from_latin25("0"), Some(0));
        assert_eq!(from_latin25("1"), Some(1));
        assert_eq!(from_latin25("2"), None);
        assert_eq!(from_latin25("3"), Some(2));
        assert_eq!(from_latin25("Z"), Some(24));
        assert_eq!(from_latin25("10"), Some(25));

        assert_eq!(from_latin25("M000000"), Some(3662109375));
    }

    #[test]
    fn check_round_trip_latin25() {
        fn prop_latin25(n: u64) -> bool {
            match from_latin25(&to_latin25(n)) {
                Some(decoded) => n == decoded,
                None => false,
            }
        }
        quickcheck(prop_latin25 as fn(u64) -> bool);
    }

    #[test]
    fn check_hash_latin25() {
        assert_eq!(
            hash_string_to_latin25("You'll get used to it. Or, you'll have a psychotic episode"),
            "E4AJ4G9E0T8Z8T"
        );
    }
}
