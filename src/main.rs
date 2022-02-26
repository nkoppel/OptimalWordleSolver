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
    // let words =
         // GUESS_HINT_TABLE[GUESS_MAP["salet"]][hint_from_str("_____") as usize].clone() &
        // &GUESS_HINT_TABLE[GUESS_MAP["courd"]][hint_from_str("____g") as usize].clone();
    let words = BitSet::ones(SOLUTION_WORDS.len());
    // let mut words = BitSet::zeros(SOLUTION_WORDS.len());

    // words.add(0);
    // words.add(1);
    // words.add(2);

    // println!("{:?}", words.iter().map(|i| SOLUTION_WORDS[i].clone()).collect::<Vec<_>>());

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
