// Utility - read a hex string into a byte vector
// This may crash due to unwraps, but given that all of the inputs from Cryptopals are well-formed
// to begin with, practically this isn't a problem.
pub fn hex_str_to_byte_vec(s: &str) -> Vec<u8> {
    let mut buf = String::new();
    let mut out: Vec<u8> = vec![];
    for c in s.chars() {
        if buf.len() == 2 {
            out.push(i64::from_str_radix(&buf, 16).unwrap() as u8);
            buf = String::new();
        }
        buf.push(c);
    }
    out.push(i64::from_str_radix(&buf, 16).unwrap() as u8);
    out
}

// Utility - compute the Hamming distance between two strings in terms of different bits
pub fn hamming_dist(s1: &str, s2: &str) -> usize {
    let (bytes1, bytes2) = (s1.as_bytes(), s2.as_bytes());
    let mut diff = 0;
    for (b1, b2) in bytes1.iter().zip(bytes2.iter()) {
        let xor = b1 ^ b2;
        for i in 0..8 {
            diff += ((xor >> i) & 0x01) as usize;
        }
    }
    diff
}

