use crate::dictionary::Dictionary;
use crate::letter::{self, is_letter};

pub fn solve(dict: &Dictionary, input: &str) -> String {
    let mut bytes_mut = input.to_owned().to_lowercase().into_bytes();

    let mut highest_score = 0;
    let mut best_shift: usize = 0;
    for shift in 0usize..26 {
        for byte in bytes_mut.iter_mut() {
            if is_letter(*byte) {
                *byte = letter::shift(*byte, 1);
            }
        }
        
        let score = dict.english_score(&bytes_mut);
        if score > highest_score {
            highest_score = score;
            best_shift = shift;
        }
    }

    for byte in bytes_mut.iter_mut() {
        if is_letter(*byte) {
            *byte = letter::shift(*byte, best_shift + 1);
        }
    }

    String::from_utf8(bytes_mut).unwrap()
}