//! Locators humans can exchange when speaking aloud.
//!
//! This is a symbol set with sixteen uniquely pronounceable digits. English16
//! was somewhat inspired by the record locators used by the civilian air
//! travel industry, but with the restriction that the symbol set is carefully
//! chosen (aviation locators do heroic things like excluding 'I' but not much
//! else).
//!
//! The fact there are sixteen symbols is more an indication of a certain
//! degree of bullheaded-ness on the part of the author, and less of any kind
//! of actual requirement. We might have a slightly better readback score if we
//! dropped to 15 or 14 unique characters. It does mean you can match up with
//! hexadecimal, which is not entirely without merit.
//!
//! The grouping of letters and numbers was the hard part; having come up with
//! the set and deconflicted the choices, the ordering is then entirely
//! arbitrary. Since there are some numbers, might as well have them at the
//! same place they correspond to in base 10; the letters were then allocated
//! in alpha order in the remaining slots.
//!
//! The symbol set is as follows:
//!
//! | Digit  | Ordinal | Phoneme | Selection Notes |
//! |:-:|:-|:-|:------|
//! | **`'0'`** | _0th_ | \[ˈziˈro\] | `'0'` Conflicts with `'O'` obviously, and `'Q'` often enough. |
//! | **`'1'`** | _1st_ | \['wan\] | |
//! | **`'2'`** | _2nd_ | /u:/ | `'U'`, `'W'`, and `'2'`. `'W'` is disqualified because of the way Australians butcher double-this and triple-that. \"Double U" or \"W\"? Who would know. |
//! | **`'C'`** | _3rd_ | /eː/ | `'B'`, `'C'`, `'D'`, `'E'`, `'G'`, `'P'`, `'T'`, `'V'`, and `'3'` (plus `'Z'` because Americans can't pronounce \"Zed\" properly). |
//! | **`'4'`** | _4th_ | \['foa\] | `'4'` and `'5'` are often confused, and '5' is definitely out due to its collision with `'I'` when spoken, and `'S'` in writing. |
//! | **`'F'`** | _5th_ | /f/ | `'F'` and `'S'` are notoriously confused when spoken aloud, making the choice of `'F'` borderline, but `'S'` is already disqualified for looking like `'5'`. |
//! | **`'H'`** | _6th_ | /h/ | chosen from `'H'` and `'8'`. The sixth ordinal is `'6'` knocked out by choice of `'X'` below |
//! | **`'7'`** | _7th_ | \[ˈsɛv.n\] | chosen from `'7'` and `'N'`, the former's additional sounds giving it a bit of additional aural distinction. |
//! | **`'8'`** | _8th_ | \[ˈei̯t\] | Uncomfortably close to `'H'` above, but the pronunciation _is_ distinct. |
//! | **`'9'`** | _9th_ | \[ˈnaɪn\] | |
//! | **`'K'`** | _10th_ | /eɪ/ | `'A'`, `'J'`, `'K'`. |
//! | **`'L'`** | _11th_ | \[el\] | `'L'` has good phonetics, and as long as it is upper case (which the whole English16 symbol set is) there's no conflict with `'1'`. |
//! | **`'M'`** | _12th_ | \[em\] | chosen from `'M'` and `'N'`, which while phonetically distinct, seem to get frequently confused in readbacks as they both hold the consonant sound and can be confused. |
//! | **`'R'`** | _13th_ | \[r\] | chef-kiss, no notes |
//! | **`'X'`** | _14th_ | /x/ | chosen from `'X'` and `'6'`. |
//! | **`'Y'`** | _15th_ | /aɪ/ | chosen from `'I'`, `'Y'`, and `'5'`. `'I'` is out for the usual reason of being similar to `'1'`. |

use crate::greater_than;

pub type English16 = String;

/// Given a number, convert it to a string in the English16 base 16 symbol
/// alphabet.
///
/// You can use this as a replacement for the standard `'0'`-`'9'` and
/// `'A'`-`'F'` symbols traditionally used to express hexadecimal, though
/// really the fact that we came up with 16 total unique symbols was a nice
/// co-incidence, not a requirement.
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

/// Given a number encoded in the English16 alphabet, convert it back to a
/// base 10 decimal.
pub fn from_english16(s: &str) -> Option<u32> {
    let mut result = 0;

    if greater_than(s, "YYYYYYYY") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

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
        assert_eq!(from_english16("YYYYYYYZ"), None); // illegal character, also "greater"
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
}
