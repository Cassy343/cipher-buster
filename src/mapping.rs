use crate::dictionary::Dictionary;
use crate::letter::c2i;

pub fn solve(dict: &Dictionary, input: &str, presets: Option<Vec<(char, char)>>) -> Option<String> {
    let mut words = input.split(' ')
        .map(|word| word.to_lowercase().into_bytes())
        .collect::<Vec<_>>();
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    
    let mut initial_mapping = [0; 26];
    for (cipher, plain) in presets.iter().flatten() {
        initial_mapping[c2i(*cipher as u8)] = *plain as u8;
    }

    let mapping = random_mapping_internal(dict, &words, initial_mapping);
    mapping.map(|key| {
        input.split(' ')
            .map(|word| word.to_lowercase().into_bytes())
            .map(|word| String::from_utf8(word.iter().map(|&ch| key[c2i(ch)]).collect()).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn random_mapping_internal(dict: &Dictionary, words: &[Vec<u8>], mapping: [u8; 26]) -> Option<[u8; 26]> {
    if words.is_empty() {
        return Some(mapping);
    }

    let word = &words[0];
    for &choice in dict.words_of_length(word.len()) {
        let mut new_mapping = mapping.clone();

        for (&cipher, &plain) in word.iter().zip(choice.as_bytes().iter()) {
            let index = c2i(cipher);
            if new_mapping[index] == 0 {
                new_mapping[index] = plain;
            }
        }

        if word.iter().map(|&ch| new_mapping[c2i(ch)]).zip(choice.as_bytes().iter().copied()).all(|(a, b)| a == b) {
            match random_mapping_internal(dict, &words[1..], new_mapping) {
                Some(mapping) => return Some(mapping),
                _ => {}
            }
        }
    }

    None
}