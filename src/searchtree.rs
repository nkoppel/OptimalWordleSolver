use crate::words::*;
use crate::word_sets::*;

use std::collections::{HashMap, HashSet};

pub fn avg_remaining_turns(words: usize) -> f64 {
    let mut words = words as f64;

    let mut out = 0.;
    let mut denom = 1.;
    let mut i = 1.;

    while words > 1. {
        out += i / words;
        denom -= 1. / words;

        words /= HINT_POSSIBILITIES as f64;
        i += 1.;
    }

    out + i * denom
}

// returns the location of the minimum value in an iterator
// if equal elements are present, returns the first occurrence
fn loc_of_min<I: Iterator<Item=T>, T: PartialOrd>(iter: I) -> Option<(usize, T)> {
    iter
        .enumerate()
        .reduce(|(i1, x1), (i2, x2)| {
            if x1 < x2 {
                (i1, x1)
            } else {
                (i2, x2)
            }
        })
}

#[derive(Clone, Debug)]
pub struct BestNode {
    pub(crate) best_guess: usize,
    pub(crate) turns: f64,
    pub(crate) words: BitSet,

    pub(crate) branches: Vec<AvgNode>
}

#[derive(Clone, Debug)]
pub enum AvgNode {
    Turns(f64),
    Node {
        turns: f64,

        hint_ordering: Vec<u8>,
        next_branch: usize,
        branches: Vec<BestNode>,
    }
}

use AvgNode::*;

macro_rules! node_field {
    ($field:ident, $func2:ident, $result:ty) => {
        pub fn $field(&self) -> &$result {
            match self {
                Node{$field: x, ..} => x,
                _ => panic!()
            }
        }

        pub fn $func2(&mut self) -> &mut $result {
            match self {
                Node{$field: x, ..} => x,
                _ => panic!()
            }
        }

    }
}

impl AvgNode {
    pub fn new() -> Self {
        Turns(0.)
    }

    pub fn turns(&self) -> f64 {
        match self {
            Turns(t) | Node{turns: t, ..} => *t
        }
    }

    pub fn turns_mut(&mut self) -> &mut f64 {
        match self {
            Turns(t) | Node{turns: t, ..} => t
        }
    }

    node_field!(hint_ordering, hint_ordering_mut, Vec<u8>);
    node_field!(next_branch, next_branch_mut, usize);
    node_field!(branches, branches_mut, Vec<BestNode>);

    pub fn init_turns(&mut self, guess: usize, parent_words: &BitSet) {
        let mut guess_average = 0.;
        let mut guess_denom = 0.;

        let mut words = BitSet::new();

        for hint in 0..HINT_POSSIBILITIES {
            if GUESS_HINT_TABLE[guess][hint].len() != 0 {
                words.clone_from(parent_words);
                words &= &GUESS_HINT_TABLE[guess][hint];

                let weight = words.count_ones() as f64;

                guess_denom += weight;
                guess_average += avg_remaining_turns(weight as usize) * weight;
            }
        }

        // if GUESS_WORDS[guess] == "trace" {
            // println!("{:x?} {} {}", parent_words, guess_average, guess_denom);
            // let mut full = BitSet::zeros(SOLUTION_WORDS.len());

            // for set in &GUESS_HINT_TABLE[guess] {
                // if set.len() > 0 {
                    // full |= set;
                // }
            // }

            // println!("{}", full.count_ones());
            // panic!();
        // }

        guess_average /= guess_denom;
        guess_average += 1.;

        match self {
            Turns(t) | Node{turns: t, ..} => *t = guess_average
        }
    }

    pub fn init_hint_ordering(&mut self, guess: usize, parent_words: &BitSet) {
        if let Turns(turns) = self {
            *self = Node {
                turns: *turns,
                hint_ordering: Vec::new(),
                next_branch: 0,
                branches: Vec::new()
            }
        } else {
            return;
        }

        let mut words = BitSet::new();
        let mut hints = Vec::new();

        for hint in 0..HINT_POSSIBILITIES {
            if GUESS_HINT_TABLE[guess][hint].len() != 0 {
                words.clone_from(parent_words);
                words &= &GUESS_HINT_TABLE[guess][hint];

                let weight = words.count_ones();

                if weight > 0 {
                    hints.push((weight, hint));
                }
            }
        }

        hints.sort_unstable();
        *self.hint_ordering_mut() = hints.into_iter().map(|x| x.1 as u8).collect();
    }

    pub fn update_turns(&mut self, guess: usize, parent_words: &BitSet) {
        let mut guess_average = 0.;
        let mut guess_denom = 0.;

        let mut words = BitSet::new();

        for (i, hint) in self.hint_ordering().iter().enumerate() {
            words.clone_from(parent_words);
            words &= &GUESS_HINT_TABLE[guess][*hint as usize];

            let weight = words.count_ones() as f64;

            guess_denom += weight;

            if i < self.branches().len() {
                guess_average += (self.branches()[i].turns + 1.) * weight;
            } else {
                guess_average += (avg_remaining_turns(weight as usize) + 1.) * weight;
            }
        }

        *self.turns_mut() = guess_average / guess_denom;
    }

    pub fn complete(&self) -> bool {
        if let Turns(s) = self {
            false
        } else {
            self.branches().len() == self.hint_ordering().len()
                && self.hint_ordering().len() > 0
        }
    }
}

impl BestNode {
    // computes the number of turns remaining and the best word from "guess_turns"
    pub fn update_turns(&mut self) {
        let (loc, min) =
            loc_of_min(self.branches.iter().map(|b| b.turns())).unwrap();

        self.best_guess = loc;
        self.turns = min;
    }

    pub fn new(words: BitSet) -> Self {
        let mut out =
            Self {
                best_guess: 0,
                turns: 0.,
                words,
                branches: vec![AvgNode::new(); GUESS_WORDS.len()],
            };

        for (guess, b) in out.branches.iter_mut().enumerate() {
            b.init_turns(guess, &out.words);
        }

        out.update_turns();

        out
    }
}
