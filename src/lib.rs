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
