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

use std::fs::File;

fn main() {
    let words = BitSet::ones(SOLUTION_WORDS.len());

    let mut tree = BestNode::new(&words);

    for i in 0.. {
        print!("{} {} ", i, tree.turns);
        if !tree.search(words.clone()) {
            break;
        }
    }

    tree.write_strategy(words.clone(), "", &mut File::create("full_solution.txt").unwrap());
    println!("{}", tree.turns);
}
