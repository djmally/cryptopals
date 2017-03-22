use std::str;
use std::u8;

use challenge2;
use utils;

// Challenge 3 Utilty
pub fn fixed_xor_all_ascii(buf: &[u8]) -> Vec<(u8, String)> {
    let mut out = Vec::new();
    for c in 0..u8::MAX {
        let single_char_buf = vec![c; buf.len()];
        if let Ok(s) = str::from_utf8(&challenge2::fixed_xor(buf, &single_char_buf)) {
            out.push((c, String::from(s)))
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Challenge 3
    #[test]
    fn challenge_3() {
        let input = utils::hex_str_to_byte_vec("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        for (c, s) in fixed_xor_all_ascii(&input) {
            if c == 88 {
                println!("{}, {}", c, s);
            }
        }
    }
}
