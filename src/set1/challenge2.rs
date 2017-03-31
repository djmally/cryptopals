// Challenge 2
// Computes the bitwise XOR of two byte slices
pub fn xor_bufs(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    assert_eq!(buf1.len(), buf2.len());

    buf1.iter().zip(buf2.iter())
        .fold(Vec::with_capacity(buf1.len()), |mut acc, (&b1, &b2)| { acc.push(b1 ^ b2); acc })
}


#[cfg(test)]
mod tests {
    use super::*;
    use set1::utils;

    #[test]
    fn test_xor_bufs() {
        let input1 = &utils::hex_str_to_byte_vec("1c0111001f010100061a024b53535009181c");
        let input2 = &utils::hex_str_to_byte_vec("686974207468652062756c6c277320657965");
        let output = utils::hex_str_to_byte_vec("746865206b696420646f6e277420706c6179");
        let result = xor_bufs(input1, input2);

        assert_eq!(output, result);

        let input1 = vec![0xFF; 32];
        let output = vec![0x00; 32];
        let result = xor_bufs(&input1, &input1);

        assert_eq!(output, result);

        let input1 = vec![0xFF; 32];
        let input2 = vec![0x00; 32];
        let output = vec![0xFF; 32];
        let result = xor_bufs(&input1, &input2);

        assert_eq!(output, result);
    }
}
