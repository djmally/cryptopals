use set1::utils;

const ENCODING: &'static [u8; 65] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const MASK: u32 = 0x3F;
const STEP: usize = 3;

// Challenge 1
// Converts the input bytes into a base64-encoded String
pub fn base64(bytes: &[u8]) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_three_spaces() {
        let input = "   ";
        let output = "ICAg";
        let result = base64(input.as_bytes());

        assert_eq!(output, &result);
    }

    #[test]
    fn base64_sample_from_cryptopals() {
        let input = &utils::hex_str_to_byte_vec("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result = base64(input);

        assert_eq!(output, &result);
    }

    #[test]
    fn base64_sample_from_wikipedia_1() {
        let input = "Man";
        let output = "TWFu";
        let result = base64(input.as_bytes());

        assert_eq!(output, &result);
    }

}
