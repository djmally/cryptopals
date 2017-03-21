#![feature(step_by)]

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

const ENCODING: &'static [u8; 65] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const MASK: u32 = 0x3F;
const STEP: usize = 3;

// Utility - read a hex string into a byte vector
fn hex_str_to_byte_vec(s: &str) -> Vec<u8> {
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
fn hamming_dist(s1: &str, s2: &str) -> usize {
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

// Challenge 1
pub fn h2b(bytes: &[u8]) -> String {
    let mut bytes: Vec<u8> = bytes.iter().cloned().collect();

    // Pad the byte string's length to a multiple of 3
    let pad = bytes.len() % STEP;
    for _ in 0..pad {
        bytes.push(0u8);
    }

    // Iterate bytes in groups of 3
    (0..bytes.len()).step_by(STEP).fold(String::with_capacity(bytes.len()), |mut acc, i| {
        let mut block: u32 = 0;
        // Concatenate bytes
        block |= (bytes[i as usize]   as u32) << 16;
        block |= (bytes[(i+1) as usize] as u32) << 8;
        block |= (bytes[(i+2) as usize] as u32) << 0;
        // Compute indices
        let i0 = (block >> 18) & MASK;
        let i1 = (block >> 12) & MASK;
        let i2 = (block >> 6) & MASK;
        let i3 = (block >> 0) & MASK;
        // Select chars
        acc.push(ENCODING[i0 as usize] as char);
        acc.push(ENCODING[i1 as usize] as char);
        acc.push(ENCODING[i2 as usize] as char);
        acc.push(ENCODING[i3 as usize] as char);
        acc
    })
}

// Challenge 2
pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    assert_eq!(buf1.len(), buf2.len());

    buf1.iter().zip(buf2.iter()).fold(Vec::new(), |mut acc, (&b1, &b2)| { acc.push(b1 ^ b2); acc })
}

// Challenge 3 Utilty
pub fn fixed_xor_all_ascii(buf: &[u8]) -> Vec<(u8, String)> {
    let mut out = Vec::new();
    for c in 0..std::u8::MAX {
        let single_char_buf = vec![c; buf.len()];
        if let Ok(s) = std::str::from_utf8(&fixed_xor(buf, &single_char_buf)) {
            out.push((c, String::from(s)))
        }
    }
    out
}

// Challenge 5
pub fn repeating_key_xor(buf: &str, key: &str) -> Vec<u8> {
    let (bytes, key_bytes): (&[u8], &[u8]) = (buf.as_bytes(), key.as_bytes());
    let mut out = Vec::with_capacity(bytes.len());

    for (idx, byte) in bytes.iter().enumerate() {
        let xor = byte ^ (key_bytes[idx % key_bytes.len()]);
        out.push(xor);

    }

    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h2b_three_spaces() {
        let input = "   ";
        let output = "ICAg";
        let result = h2b(input.as_bytes());

        assert_eq!(output, &result);
    }

    #[test]
    fn h2b_sample_from_cryptopals() {
        let input = &hex_str_to_byte_vec("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result = h2b(input);

        assert_eq!(output, &result);
    }

    #[test]
    fn h2b_sample_from_wikipedia_1() {
        let input = "Man";
        let output = "TWFu";
        let result = h2b(input.as_bytes());

        assert_eq!(output, &result);
    }

    #[test]
    fn fixed_xor_1() {
        let input1 = &hex_str_to_byte_vec("1c0111001f010100061a024b53535009181c");
        let input2 = &hex_str_to_byte_vec("686974207468652062756c6c277320657965");
        let output = hex_str_to_byte_vec("746865206b696420646f6e277420706c6179");
        let result = fixed_xor(input1, input2);

        assert_eq!(output, result);
    }

    // Challenge 3
    #[test]
    fn challenge_3() {
        let input = hex_str_to_byte_vec("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        for (c, s) in fixed_xor_all_ascii(&input) {
            if c == 88 {
                println!("{}, {}", c, s);
            }
        }
    }

    #[test]
    fn challenge_4() {
        let f = File::open("data/4.txt").unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.unwrap();
            let input = hex_str_to_byte_vec(&line);
            for (c, s) in fixed_xor_all_ascii(&input) {
                if c == 53 {
                    println!("{}, {}", c, s);
                }
            }
        }
    }

    #[test]
    fn challenge_5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let output = hex_str_to_byte_vec("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

        let result = repeating_key_xor(input, key);

        assert_eq!(output.len(), result.len());
        for (o, r) in output.iter().zip(result.iter()) {
            assert_eq!(o, r);
        }
    }

    #[test]
    fn test_hamming_dist_1() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let output = 37;

        let result = hamming_dist(s1, s2);

        assert_eq!(output, result);
    }
}
