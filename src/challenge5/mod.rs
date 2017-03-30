
// Challenge 5
pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes.iter().enumerate().fold(Vec::with_capacity(bytes.len()), |mut acc, (idx, byte)| {
        acc.push(byte ^ (key[idx % key.len()]));
        acc
     })
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils;

    #[test]
    fn challenge_5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let output = utils::hex_str_to_byte_vec("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

        let result = repeating_key_xor(input.as_bytes(), key.as_bytes());

        assert_eq!(output.len(), result.len());
        for (o, r) in output.iter().zip(result.iter()) {
            assert_eq!(o, r);
        }
    }
}
