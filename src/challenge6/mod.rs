
#[cfg(test)]
mod tests {
    use super::*;
    use utils;

    #[test]
    fn test_hamming_dist_1() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let output = 37;

        let result = utils::hamming_dist(s1, s2);

        assert_eq!(output, result);
    }
}
