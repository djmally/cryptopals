use std::str;
use std::u8;

use challenge2;
use utils;

// Challenge 3 Utilty
pub fn fixed_xor_all_ascii(buf: &[u8]) -> Vec<Vec<u8>> {
    (0..u8::MAX).fold(Vec::with_capacity(buf.len()), |mut acc, byte| {
        acc.push(challenge2::fixed_xor(buf, &vec![byte; buf.len()]));
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Challenge 3
    #[test]
    fn challenge_3() {
        let input = utils::hex_str_to_byte_vec("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let byte_vecs = fixed_xor_all_ascii(&input);

        let most = utils::most_ascii(byte_vecs.as_slice());

        println!("{:?}", most);
        assert!(most.iter().any(|&(key, _)| key == 88));
    }
}
