use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_candidate_word(w: &str) -> bool {
    let bytes = w.as_bytes();
    if bytes.len() != 5 {
	return false;
    }
    if bytes.iter().any(|c| *c > b'z' || *c < b'a') {
	return false;
    }
    // Short words, I assume dumb is fast.
    for i in 0..bytes.len() {
	for j in (i+1)..bytes.len() {
	    if bytes[i] == bytes[j] {
		return false;
	    }
	}
    }

    true
}

// Build a bitmask representing the letters the word uses.
fn to_bitmask(w: &str) -> u32 {
    let mut mask: u32 = 0;
    for c in w.bytes() {
	mask |= 1 << (c - b'a');
    }
    mask
}

// Build a list of bitmasks, with the words associated with each mask.
fn build_word_list<'a>(ws: Vec<String>) -> Vec<(u32, Vec<String>)> {
    let mut map = HashMap::new();
    for w in ws.into_iter() {
	let mask = to_bitmask(&w);
	let entry = map.entry(mask).or_insert_with(|| Vec::new());
	entry.push(w);
    }
    let mut v: Vec<(u32, Vec<String>)> = map.into_iter().collect::<Vec<_>>();
    v.sort_unstable();
    v
}

fn main() {
    // Extract potential usable words from the input file.
    let file = File::open("words_alpha.txt").unwrap();
    let words = BufReader::new(file).lines().map(|l| l.unwrap());
    let candidate_words = words.filter(|w| is_candidate_word(w)).collect::<Vec<String>>();
    eprintln!("{} candidate words", candidate_words.len());

    let word_list = build_word_list(candidate_words);
    eprintln!("{:?}", word_list);
}
