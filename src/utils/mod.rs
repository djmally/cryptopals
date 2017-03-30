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
        (c >= 0x61 && c <= 0x79) {
            acc += 1
        }
        acc
    })
}

// Utility - find the byte vector(s) with the largest number of ASCII characters, with a tolerance
// of being within 2 of the max. Returns a vector of (u8, &String) tuples, where the u8 is
// the key used to decrypt the string
pub fn most_ascii<'a>(slices: &[Vec<u8>]) -> Vec<(u8, &Vec<u8>)> {
    let score: HashMap<(u8, &Vec<u8>), usize> = slices.iter().enumerate().map(|(idx, slice)| ((idx as u8, slice), count_ascii(slice))).collect();

    // TODO? Remove this expect
    let max = *(score.iter().max_by_key(|&(ref k, v)| v).expect("No max found").1);

    let (keys, _): (Vec<(u8, &Vec<u8>)>, Vec<_>) = score.into_iter().filter(|&(ref k, v)| v <= max && v >= max - 2).unzip();
    keys
}
