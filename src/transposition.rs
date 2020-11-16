use crate::Dictionary;
use itertools::Itertools;

pub fn solve(dict: &Dictionary, input: &str) -> String {
    let len = input.len();
    let transpositions = factor_pairs(len);
    let input = input.as_bytes();
    let mut buffer = vec![0; len];

    let mut highest_score = 0;
    let mut best_transposition = (0, 0);
    for key in transpositions {
        transpose(input, &mut buffer, key);

        let score = dict.english_score(&buffer);
        if score > highest_score {
            highest_score = score;
            best_transposition = key;
        }
    }

    transpose(input, &mut buffer, best_transposition);
    String::from_utf8(buffer).unwrap()
}

fn transpose(src: &[u8], dest: &mut [u8], key: (usize, usize)) {
    for i in 0..key.0 {
        for j in 0..key.1 {
            dest[i * key.1 + j] = src[j * key.0 + i];
        }
    }
}

fn factor_pairs(x: usize) -> Vec<(usize, usize)> {
    let mut div = 1;
    let mut factor_pairs = Vec::new();
    while div <= x {
        if x % div == 0 {
            factor_pairs.push((div, x / div));
        }
        
        div += 1;
    }

    factor_pairs
}