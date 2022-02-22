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
    // println!("{:?}", gen_guess_hint_table()[GUESS_MAP["trace"]]
        // .iter()
        // .map(|x| {
            // let weight = x.clone().count_ones();

            // avg_remaining_turns(weight) * weight as f64 / SOLUTION_WORDS.len() as f64
        // })
        // .sum::<f64>() + 1.
    // );

    // for i in 0..100 {
        // let tree = BestNode::new(BitSet::ones(SOLUTION_WORDS.len()));

        // println!("{} {}", i, GUESS_WORDS[tree.best_guess]);
    // }

    let mut tree = BestNode::new(BitSet::ones(SOLUTION_WORDS.len()));

    for i in 0.. {
        print!("{} {} ", i, tree.turns);
        tree.search();
    }

    let mut guess_turns =
        tree.branches
            .iter()
            .map(|b| b.turns())
            .enumerate()
            .collect::<Vec<_>>();

    guess_turns.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

    for (word, turns) in guess_turns {
        println!("{} {}", GUESS_WORDS[word], turns);
    }
}
