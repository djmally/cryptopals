
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::fs::File;

    use challenge3;
    use utils;

    #[test]
    fn challenge_4() {
        let f = File::open("data/4.txt").unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.unwrap();
            let input = utils::hex_str_to_byte_vec(&line);
            for (c, s) in challenge3::fixed_xor_all_ascii(&input) {
                if c == 53 {
                    println!("{}, {}", c, s);
                }
            }
        }
    }
}
