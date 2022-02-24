#![allow(dead_code)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;

mod words;
mod word_sets;
mod searchtree;
mod search;

use words::*;
use word_sets::*;
use searchtree::*;
use search::*;

fn main() {
    // let mut list = Vec::new();

    // for i in 0..GUESS_WORDS.len() {
        // list.push((avg_entropy(0..SOLUTION_WORDS.len(), i), i));
    // }

    // list.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    // for (turns, w) in list {
        // println!("{} {}", GUESS_WORDS[w], turns);
    // }

    // for i in 0..100 {
        // let tree = BestNode::new(BitSet::ones(SOLUTION_WORDS.len()));

        // println!("{} {}", i, GUESS_WORDS[tree.best_guess]);
    // }

    let all_words = BitSet::ones(SOLUTION_WORDS.len());

    let mut tree = BestNode::new(&all_words);

    println!("{}", tree.entropy);
    println!("{}", tree.branches[tree.best_guess].as_ref().unwrap_err());

    for i in 0.. {
        print!("{} {} ", i, tree.entropy);
        tree.search();
    }

    // let mut guess_turns =
        // tree.branches
            // .iter()
            // .map(|b| b.turns())
            // .enumerate()
            // .collect::<Vec<_>>();

    // guess_turns.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

    // for (word, turns) in guess_turns {
        // println!("{} {}", GUESS_WORDS[word], turns);
    // }
}
