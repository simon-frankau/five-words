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
fn build_word_list<'a>(ws: Vec<String>) -> HashMap<u32, Vec<String>> {
    let mut map = HashMap::new();
    for w in ws.into_iter() {
	let mask = to_bitmask(&w);
	let entry = map.entry(mask).or_insert_with(|| Vec::new());
	entry.push(w);
    }
    map
}

// Given a bitmask of used letters and a range of masks representing
// words, fill in a vector with those masks that are compatible.
fn candidates(used: u32, candidates: &[u32], output: &mut Vec<u32>) {
    output.clear();
    for candidate in candidates.iter() {
	if candidate & used == 0 {
	    output.push(*candidate);
	}
    }
}

fn solve(words: &[u32]) -> Vec<[u32; 5]> {
    let mut words2 = Vec::with_capacity(words.len());
    let mut words3 = Vec::with_capacity(words.len());
    let mut words4 = Vec::with_capacity(words.len());
    let mut words5 = Vec::with_capacity(words.len());

    let mut res = Vec::new();
    for (idx, word) in words.iter().enumerate() {
	candidates(*word, &words[idx + 1..], &mut words2);
	for (idx2, word2) in words2.iter().enumerate() {
	    candidates(*word2, &words2[idx2 + 1..], &mut words3);
	    for (idx3, word3) in words3.iter().enumerate() {
		candidates(*word3, &words3[idx3 + 1..], &mut words4);
		for (idx4, word4) in words4.iter().enumerate() {
		    candidates(*word4, &words4[idx4 + 1..], &mut words5);
		    for word5 in words5.iter() {
			res.push([*word, *word2, *word3, *word4, *word5]);
		    }
		}
	    }
	}
    }
    res
}

// Print all the word combinations associated with a specific bitmask
// solution.
fn print_solution(solution: &[u32; 5], map: &HashMap<u32, Vec<String>>) {
    for w1 in map.get(&solution[0]).unwrap().iter() {
	for w2 in map.get(&solution[1]).unwrap().iter() {
	    for w3 in map.get(&solution[2]).unwrap().iter() {
		for w4 in map.get(&solution[3]).unwrap().iter() {
		    for w5 in map.get(&solution[4]).unwrap().iter() {
			println!("{}, {}, {}, {}, {}", w1, w2, w3, w4, w5);
		    }
		}
	    }
	}
    }
}

fn main() {
    // Extract potential usable words from the input file.
    let file = File::open("words_alpha.txt").unwrap();
    let words = BufReader::new(file).lines().map(|l| l.unwrap());
    let candidate_words = words.filter(|w| is_candidate_word(w)).collect::<Vec<String>>();
    eprintln!("{} candidate words", candidate_words.len());

    let word_list = build_word_list(candidate_words);
    eprintln!("{:?}", word_list);

    let mask_vec = word_list.keys().cloned().collect::<Vec<_>>();
    let solutions = solve(&mask_vec);
    for solution in solutions {
	print_solution(&solution, &word_list);
    }
}
