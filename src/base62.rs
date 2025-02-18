use base62;
use sha2::Digest;

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

pub fn hash_string_to_base62(s: &str) -> String {
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

    to_base62(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pad_with_zeros;

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
    fn check_left_padding() {
        assert_eq!(pad_with_zeros(5, "1"), "00001");
        assert_eq!(pad_with_zeros(5, "123456"), "123456");
        assert_eq!(pad_with_zeros(10, &to_base62(u64::MAX)), "LygHa16AHYF");
        assert_eq!(pad_with_zeros(11, &to_base62(u64::MAX)), "LygHa16AHYF");
        assert_eq!(pad_with_zeros(12, &to_base62(u64::MAX)), "0LygHa16AHYF");
    }

    #[test]
    fn check_hash_base62() {
        assert_eq!(
            hash_string_to_base62("You'll get used to it. Or, you'll have a psychotic episode"),
            "GDDR4HcGLCV"
        );
    }
}
