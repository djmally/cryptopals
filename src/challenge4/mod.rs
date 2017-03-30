
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::fs::File;
    use std::str;

    use challenge3;
    use utils;

    #[test]
    fn challenge_4() {
        let f = File::open("data/4.txt").unwrap();
        let f = BufReader::new(f);
        let mut decryption_key_present = false;

        for line in f.lines() {
            let line = line.unwrap();
            let input = utils::hex_str_to_byte_vec(&line);

            let strings = challenge3::fixed_xor_all_ascii(&input);

            for (key, s) in utils::most_ascii(strings.as_slice()) {
                if let Ok(s) = str::from_utf8(s) {
                    if key == 45 { decryption_key_present = true; }
                    println!("{}, {:?}", key, s);
                }
            }
        }
        assert!(decryption_key_present);
    }
}
