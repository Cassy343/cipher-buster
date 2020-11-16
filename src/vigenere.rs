use crate::dictionary::Dictionary;
use crate::letter::{c2i, shift};

pub fn solve(dict: &Dictionary, input: &str) -> String {
    let ciphertext = input.to_lowercase().into_bytes();
    let mut plaintext = vec![0; ciphertext.len()];

    let mut max_score = 0;
    let mut result = "".to_owned();
    for keyword in dict.words() {
        vigenere(&ciphertext, keyword.as_bytes(), &mut plaintext);

        let score = dict.english_score(&plaintext);
        if score > max_score {
            max_score = score;
            result = String::from_utf8(plaintext.clone()).unwrap();
        }
    }

    result
}

pub fn decode(input: &str, key: &str) -> String {
    let mut buffer = vec![0; input.len()];
    vigenere(input.to_lowercase().as_bytes(), key.as_bytes(), &mut buffer);
    String::from_utf8(buffer).unwrap()
}

fn vigenere(ciphertext: &[u8], key: &[u8], buffer: &mut [u8]) {
    let key_len = key.len();
    for (index, plain_ch) in buffer.iter_mut().enumerate() {
        *plain_ch = shift(ciphertext[index], 26 - c2i(key[index % key_len]));
    }
}