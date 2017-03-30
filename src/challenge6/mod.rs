use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::mem;

use nalgebra::DMatrix;
use nalgebra::dimension::{Dim, Dynamic};

use challenge3;
use utils;

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

fn smallest_3_norm_edit_dist(bytes: &[u8]) -> [usize; 3] {
    let mut norm_edit_dists: Vec<_> = (0..41).map(|i| (i, norm_edit_dist_for_keysize(i as usize, bytes))).collect();
    // Sort descending
    norm_edit_dists.sort_by(|&(ref i, ref a), &(ref j, ref b)| b.partial_cmp(a).expect("partial_cmp failed!"));
    // Return only the key sizes themselves
    [norm_edit_dists[0].0, norm_edit_dists[1].0, norm_edit_dists[2].0]
}

fn build_ciphertext_col_matrix(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    // Pad the input to a multiple of the key size
    let mut bytes: Vec<_> = bytes.iter().cloned().collect();
    while bytes.len() % key_size != 0 {
        bytes.push(0x00);
    }

    // Initialize vectors
    let ratio = bytes.len() / key_size;
    let mut out = Vec::with_capacity(ratio);
    for i in 0..ratio {
        out.push(Vec::with_capacity(ratio));
    }

    for (idx, byte) in bytes.into_iter().enumerate() {
        println!("{:?}", byte);
        out[idx % ratio][idx] = byte;
    }

    out
}

fn build_ciphertext_matrices(file_name: &str) -> [Vec<Vec<u8>>; 3] {
    let f = File::open(file_name).expect("File::open failed!");
    let f = BufReader::new(f);
    let bytes: Vec<u8> = f.lines().fold(Vec::new(), |mut acc, line| { acc.push(line.expect("Line unwrap failed!").into_bytes()); acc }).into_iter().flat_map(|b| b).collect();
    let key_sizes = smallest_3_norm_edit_dist(&bytes);
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

/*fn build_ciphertext_matrices(file_name: &str, num_lines: usize, line_len: usize) -> [DMatrix<u8>; 3] {
    let f = File::open("data/6.txt").unwrap();
    let f = BufReader::new(f);
    let bytes: Vec<u8> = f.lines().fold(Vec::new(), |mut acc, line| { acc.push(line.unwrap().into_bytes()); acc }).into_iter().flat_map(|b| b).collect();
    let key_sizes = smallest_3_norm_edit_dist(&bytes);
    let matrices = [
        DMatrix::from_iterator(key_sizes[0], key_sizes[0], bytes),
        DMatrix::from_iterator(key_sizes[1], key_sizes[1], bytes),
        DMatrix::from_iterator(key_sizes[2], key_sizes[2], bytes)
    ];

    for mat in &matrices {
        decrypt_matrix(mat, key_sizes[0]);
    }
    matrices
}*/

/*fn decrypt_matrix(mat: &DMatrix<u8>, dim: usize) {
    for i in 0..dim {
        let col = mat.column(i);
        let decryptions = challenge3::fixed_xor_all_ascii(col);
        let most = utils::most_ascii(decryptions.as_slice());
        println!("{:?}", most);
    }
}*/


#[cfg(test)]
mod tests {
    use super::*;
    use utils;

    #[test]
    fn test_hamming_dist_1() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let output = 37;

        let result = utils::hamming_dist(s1.as_bytes(), s2.as_bytes());

        assert_eq!(output, result);
    }

    #[test]
    fn test_smallest_3_norm_edit_dist() {
        
    }

    #[test]
    fn test_decrypt_matrix() {
        let mut matrices = build_ciphertext_matrices("data/6.txt");
        println!("{:?}", matrices);
        decrypt_matrix(mem::replace(&mut matrices[0], Vec::new()));
        decrypt_matrix(mem::replace(&mut matrices[1], Vec::new()));
        decrypt_matrix(mem::replace(&mut matrices[2], Vec::new()));
        panic!();
    }
}
