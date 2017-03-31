use std::collections::hash_map::HashMap;

use std::ascii::AsciiExt;

// Utility - read a hex string into a byte vector
// This may crash due to unwraps, but given that all of the inputs from Cryptopals are well-formed
// to begin with, practically this isn't a problem.
pub fn hex_str_to_byte_vec(s: &str) -> Vec<u8> {
    let mut buf = String::new();
    let mut out: Vec<u8> = vec![];
    for c in s.chars() {
        if buf.len() == 2 {
            out.push(i64::from_str_radix(&buf, 16).expect("Invalid hex literal") as u8);
            buf = String::new();
        }
        buf.push(c);
    }
    while buf.len() % 2 == 1 {
        // Pad the buffer as necessary
        buf.push('0');
    }
    // Flush any remaining characters
    out.push(i64::from_str_radix(&buf, 16).expect("Invalid hex literal") as u8);
    out
}

// Utility - compute the Hamming distance between two byte strings in terms of different bits
pub fn hamming_dist(bytes1: &[u8], bytes2: &[u8]) -> usize {
    let mut diff = 0;
    for (b1, b2) in bytes1.iter().zip(bytes2.iter()) {
        let xor = b1 ^ b2;
        for i in 0..8 {
            diff += ((xor >> i) & 0x01) as usize;
        }
    }
    diff
}

// Utility - count the number of characters within the ASCII alphanumeric range in a string
pub fn count_ascii(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |mut acc, &c| {
        // space
        if c == 0x20 ||
        //  0 - 9
        (c >= 0x30 && c <= 0x39) ||
        //  A - Z
        (c >= 0x41 && c <= 0x5A) ||
        //   a - z
        (c >= 0x61 && c <= 0x7A) {
            acc += 1
        }
        acc
    })
}

// Utility - find the byte vector(s) with the largest number of ASCII characters, with a tolerance
// of being within 2 of the max. Returns a vector of (u8, &String) tuples, where the u8 is
// the key used to decrypt the string. May crash if no max is found
pub fn most_ascii<'a>(slices: &[Vec<u8>]) -> Vec<(u8, &Vec<u8>)> {
    let score: HashMap<(u8, &Vec<u8>), usize> = slices.iter().enumerate().map(|(idx, slice)| ((idx as u8, slice), count_ascii(slice))).collect();

    // TODO? Remove this expect
    let max: usize = *(score.iter().max_by_key(|&(ref k, v)| v).expect("No max found").1);

    let threshold = 2;
    if max < threshold { return vec![]; }

    let (keys, _): (Vec<(u8, &Vec<u8>)>, Vec<_>) = score.into_iter()
                    .filter(|&(ref k, v)| v <= max && v >= max - threshold).unzip();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_str_to_byte_vec() {
        let s = "0123456789ABCDEF";
        assert_eq!(hex_str_to_byte_vec(s), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);

        let s = "0123456789ABCDE";
        assert_eq!(hex_str_to_byte_vec(s), vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xE0]);
    }

    #[test]
    #[should_panic]
    fn test_hex_str_to_byte_vec_panics() {
        hex_str_to_byte_vec("xyz");
    }

    #[test]
    fn test_hamming_dist() {
        let s1 = "0";
        let s2 = "1";
        assert_eq!(1, hamming_dist(s1.as_bytes(), s2.as_bytes()));

        let s1 = "0";
        let s2 = "3";
        assert_eq!(2, hamming_dist(s1.as_bytes(), s2.as_bytes()));

        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        assert_eq!(37, hamming_dist(s1.as_bytes(), s2.as_bytes()));
    }

    #[test]
    fn test_count_ascii() {
        let input = "abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()œåΩ∑ß≈®ƒ√†©∫¥∆µø¬≤π…≥÷“æ‘«≠";
        assert_eq!(36, count_ascii(input.as_bytes()));
    }

    #[test]
    fn test_most_ascii() {
        let slices = vec![String::from("abcdefghijklm").into_bytes(),
                          String::from("nopqrstuvwxyz1234567890").into_bytes(),
                          String::from("!@#$%^&*()œ").into_bytes(),
                          String::from("åΩ∑ß≈®ƒ√†©∫¥∆µø¬≤π…≥÷“æ‘«≠").into_bytes()];
        let most = most_ascii(slices.as_slice());
        assert_eq!(most.len(), 1);
        assert_eq!(most[0], (1, &slices[1]));
    }

    #[test]
    #[should_panic]
    fn test_most_ascii_panics() {
        most_ascii(&vec![]);
    }
}
