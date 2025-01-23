pub struct Solution;

use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            is_end: false,
            children: HashMap::new(),
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current: &mut TrieNode = &mut self.root;

        for c in word.chars() {
            current = current.children.entry(c).or_insert_with(TrieNode::new);
        }

        current.is_end = true;
    }

    pub fn shortest_root(&self, word: &str) -> String {
        let mut current: &TrieNode = &self.root;

        for (i, c) in word.chars().enumerate() {
            if let Some(child) = current.children.get(&c) {
                current = child;

                if current.is_end {
                    return word[..=i].to_string();
                }
            } else {
                break;
            }
        }

        word.to_string()
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dict_trie: Trie = Trie::new();

        for word in dictionary {
            dict_trie.insert(&word);
        }

        sentence
            .split_whitespace()
            .map(|word: &str| dict_trie.shortest_root(word))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
