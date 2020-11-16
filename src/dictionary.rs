use crate::letter::{c2i, is_letter};
use std::iter;

pub const RAW_DICTIONARY: &str = include_str!("dictionary.txt");

pub struct Dictionary {
    tree: DictionaryTreeNode,
    word_list: Vec<Vec<&'static str>>
}

impl Dictionary {
    pub fn new() -> Self {
        let mut word_list = Vec::new();

        let mut root = DictionaryTreeNode::new(false);
        for word in RAW_DICTIONARY.split('\n') {
            root.add(word.as_bytes(), 0);

            while word_list.len() < word.len() {
                word_list.push(Vec::new());
            }

            word_list[word.len() - 1].push(word);
        }

        Dictionary {
            tree: root,
            word_list
        }
    }

    pub fn slurp_word<T: AsRef<[u8]> + ?Sized>(&self, word: &T, start: usize) -> usize {
        self.tree.slurp_word(word.as_ref(), start)
    }

    pub fn english_score<T: AsRef<[u8]> + ?Sized>(&self, string: &T) -> usize {
        let bytes = string.as_ref();
        let len = bytes.len();

        let mut index = 0;
        let mut score = 0;
        while index < len {
            let word_len = self.slurp_word(&bytes[index..], 0);
            index += word_len;
            score += word_len * word_len;

            while index < len && !is_letter(bytes[index]) {
                index += 1;
            }
        }

        score
    }

    pub fn words_of_length(&self, len: usize) -> &[&'static str] {
        &self.word_list[len - 1]
    }

    pub fn words(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.word_list.iter().flatten().copied()
    }
}

#[derive(Clone)]
struct DictionaryTreeNode {
    forms_word: bool,
    next: [Option<Box<DictionaryTreeNode>>; 26]
}

impl DictionaryTreeNode {
    fn new(forms_word: bool) -> Self {
        DictionaryTreeNode {
            forms_word,
            next: array_init::from_iter(iter::repeat(None)).unwrap()
        }
    }

    fn add(&mut self, word: &[u8], index: usize) {
        if index == word.len() {
            return;
        }

        let node = &mut self.next[c2i(word[index])];
        
        if node.is_none() {
            *node = Some(Box::new(DictionaryTreeNode::new(index == word.len() - 1)));
        }

        node.as_mut().unwrap().add(word, index + 1);
    }

    fn is_word(&self, word: &[u8], index: usize) -> bool {
        match &self.next[c2i(word[index])] {
            Some(node) => {
                if index == word.len() - 1 {
                    node.forms_word
                } else {
                    node.is_word(word, index + 1)
                }
            },
            None => false
        }
    }

    fn slurp_word(&self, string: &[u8], index: usize) -> usize {
        if index >= string.len() {
            return string.len();
        }

        match string[index] as char {
            ch @ 'a'..='z' => match &self.next[c2i(ch as u8)] {
                Some(node) => node.slurp_word(string, index + 1),
                None => index
            },

            _ => index
        }
        
    }
}