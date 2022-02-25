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
    let all_words = BitSet::ones(SOLUTION_WORDS.len());

    let mut tree = BestNode::new(&all_words);

    println!("{}", tree.turns);
    println!("{}", tree.branches[tree.best_guess].as_ref().unwrap_err());

    for i in 0.. {
        print!("{} {} ", i, tree.turns);
        tree.search();
    }
}
