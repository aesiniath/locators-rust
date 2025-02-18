//! # Background
//!
//! We had a need for identifiers that could be used by humans.
//!
//! The requirement to be able to say these over the phone complicates
//! matters. Most people have approached this problem by using a phonetic
//! alphabet. The trouble comes when you hear people saying stuff like "A as
//! in ... uh, Apple?" (should be Alpha, of course) and "U as in ... um,
//! what's a word that starts with U?" It gets worse. Ever been to a GPG
//! keysigning? Listen to people attempt to read out the digits of their key
//! fingerprints. ...C 3 E D 0 0 0 0 0 0 0 2 B D B D... "Did you say 'C' or
//! 'D'?" and "how many zeros was that?" Brutal.
//!
//! So what we need is a symbol set where each digit is unambigious and
//! doesn't collide with the phonetics of another symbol. This package
//! provides [English16](crate::english16), a set of 16 letters and numbers
//! that, when spoken in _English_, have unique pronounciation and have been
//! very successful in verbal communications over noisy links.
//!
//! Ironically, however, when used in written applications the English16 set
//! is a bit restrictive. When _looking_ at them they don't have much variety
//! (it turned out they're very blocky—so much so you have to squint). If the
//! application is transcription or identification visually then the criteria
//! is shapes that are distinct, rather than their sound. For these uses we
//! provide [Latin25](crate::latin25), a set of 25 symbols useful for
//! identifiers in automated systems that nevertheless have to be operated or
//! debugged by humans.
//!
//! Finally, also included is code to work in base 62, which is simply
//! `['0'-'9', 'A'-'Z', and 'a'-'z']`. These are frequently used to express
//! short codes in URL redirectors; you may find them a more useful encoding
//! for expressing numbers than base 16 hexidecimal.

pub mod base62;
pub mod english16;
pub mod latin25;

#[allow(dead_code)]
fn pad_with_zeros(width: usize, s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
