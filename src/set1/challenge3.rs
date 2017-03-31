use std::str;
use std::u8;

use set1::challenge2;
use set1::utils;

// Challenge 3 Utilty
// Computes the set of bitwise XORs of `buf` against vectors of all possible 8-bit values.
pub fn fixed_xor_all_ascii(buf: &[u8]) -> Vec<Vec<u8>> {
    (0..u8::MAX).fold(Vec::with_capacity(buf.len()), |mut acc, byte| {
        acc.push(challenge2::xor_bufs(buf, &vec![byte; buf.len()]));
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Challenge 3
    // Finds the XOR(s) with the most ASCII characters and prints them out, revealing the
    // brute-forced decryption of the input
    #[test]
    fn challenge_3() {
        let input = utils::hex_str_to_byte_vec("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let byte_vecs = fixed_xor_all_ascii(&input);

        let most = utils::most_ascii(byte_vecs.as_slice());

        println!("{:?}", most);
        assert!(most.iter().any(|&(key, _)| key == 88));
    }
}
