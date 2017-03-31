use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::mem;

use set1::challenge3;
use set1::utils;

// Computes the normalized edit distance for a given key size. Given an input slice of bytes, take
// a subslice of 2 * key size and split it at the middle. Then, compute the edit distance between
// the two blocks, and normalize it by dividing by the keysize.
fn norm_edit_dist_for_keysize(keysize: usize, bytes: &[u8]) -> f64 {
    let (block1, block2) = bytes[0..keysize * 2].split_at(keysize);
    assert_eq!(block1.len(), block2.len());

    let edit_dist = utils::hamming_dist(block1, block2);

    if keysize != 0 {
        (edit_dist as f64) / (keysize as f64)
    } else {
        0f64
    }
}

// Computes the smallest three normalized edit distances for a slice of bytes and a max key size.
fn smallest_3_norm_edit_dist(bytes: &[u8], max_keysize: usize) -> [usize; 3] {
    let mut norm_edit_dists: Vec<_> = (0..(max_keysize+1)).map(|i| (i, norm_edit_dist_for_keysize(i as usize, bytes))).collect();
    // Sort descending
    norm_edit_dists.sort_by(|&(ref i, ref a), &(ref j, ref b)| b.partial_cmp(a).expect("partial_cmp failed!"));
    // Return only the key sizes themselves
    [norm_edit_dists[0].0, norm_edit_dists[1].0, norm_edit_dists[2].0]
}

// Given a byte stream, break it into blocks of length `keysize`. Then, for each block, create a
// column vector within a vector of vectors.
fn build_ciphertext_col_matrix(bytes: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    // Pad the input to a multiple of the key size
    let mut bytes: Vec<_> = bytes.iter().cloned().collect();
    while bytes.len() % keysize != 0 {
        bytes.push(0x00);
    }

    // Initialize vectors
    let ratio = bytes.len() / keysize;
    let mut out = Vec::with_capacity(ratio);
    for i in 0..ratio {
        out.push(Vec::with_capacity(ratio));
    }

    for (idx, byte) in bytes.into_iter().enumerate() {
        out[idx % ratio].push(byte);
    }

    out
}

fn build_ciphertext_matrices(file_name: &str) -> [Vec<Vec<u8>>; 3] {
    let f = File::open(file_name).expect("File::open failed!");
    let f = BufReader::new(f);
    let bytes: Vec<u8> = f.lines().fold(Vec::new(), |mut acc, line| { acc.push(line.expect("Line unwrap failed!").into_bytes()); acc }).into_iter().flat_map(|b| b).collect();
    let key_sizes = smallest_3_norm_edit_dist(&bytes, 40);
    [
        build_ciphertext_col_matrix(&bytes, key_sizes[0]),
        build_ciphertext_col_matrix(&bytes, key_sizes[1]),
        build_ciphertext_col_matrix(&bytes, key_sizes[2]),
    ]
}

fn decrypt_matrix(mat: Vec<Vec<u8>>) {
    for row in mat {
        let decryptions = challenge3::fixed_xor_all_ascii(&row);
        let most = utils::most_ascii(&decryptions);
        println!("{:?}", most);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use set1::utils;

    #[test]
    fn test_smallest_3_norm_edit_dist() {
        
    }

    #[test]
    fn test_decrypt_matrix() {
        let mut matrices = build_ciphertext_matrices("data/6.txt");
        decrypt_matrix(mem::replace(&mut matrices[0], Vec::new()));
        decrypt_matrix(mem::replace(&mut matrices[1], Vec::new()));
        decrypt_matrix(mem::replace(&mut matrices[2], Vec::new()));
        panic!();
    }
}
