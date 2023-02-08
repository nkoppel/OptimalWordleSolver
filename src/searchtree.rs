use crate::words::*;
use crate::word_sets::*;

use std::cell::RefCell;

pub const HINT_POSSIBILITIES: usize = 243;

// represents the hint which indicates that the correct
// word has been guessed and the game is over
pub const ALL_GREEN: usize = HINT_POSSIBILITIES - 1;

// pub fn avg_turns(words: usize) -> f64 {
    // let mut words = words as f64;

    // if words == 0. {
        // return 0.;
    // }

    // let mut out = 0.;
    // let mut denom = 1.;
    // let mut i = 1.;

    // while words > 1. {
        // out += i / words;
        // denom -= 1. / words;

        // words /= HINT_POSSIBILITIES as f64;
        // i += 1.;
    // }

    // out + i * denom
// }

pub fn avg_turns(mut words: usize) -> usize {
    if words == 0 {
        return 0;
    }

    let mut out = 0;
    let mut factor = 1;
    let mut i = 1;

    while factor < words {
        words -= factor;
        out += i * factor;
        factor *= HINT_POSSIBILITIES;
        i += 1;
    }

    out + i * words
}

fn get_hint_frequency<I: Iterator<Item=usize>>(buf: &mut Vec<usize>, words: I, guess: usize) {
    for w in words {
        buf[TABLE[guess][w] as usize] += 1;
    }
}

pub fn weighted_average<I, J>(weights: I, nums: J) -> f64
    where I: Iterator<Item=f64>, J: Iterator<Item=f64>
{
    let mut out = 0.;
    let mut sum = 0.;

    for (weight, num) in weights.zip(nums) {
        out += weight * num;
        sum += weight;
    }

    if sum == 0. {
        0.
    } else {
        out / sum
    }
}

thread_local!(static BUFFER: RefCell<Vec<usize>> = RefCell::new(vec![0; HINT_POSSIBILITIES]));

pub fn guess_turns<I: Iterator<Item=usize>>(words: I, guess: usize) -> usize {
    let mut out = 0;

    BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();

        buf.fill(0);

        get_hint_frequency(&mut buf, words, guess);

        out = buf
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if i == ALL_GREEN {
                    0
                } else {
                    avg_turns(*w)
                }
            })
            .sum();
    });

    out
}

// returns the location of the minimum value in an iterator
// if equal elements are present, returns the first occurrence
fn loc_of_min<I, T>(iter: I) -> Option<(usize, T)>
    where I: Iterator<Item=T>, T: PartialOrd
{
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

pub fn best_turns(words: &WordSet) -> (usize, usize) {
    let words = words.iter().collect::<Vec<_>>();

    loc_of_min (
        (0..GUESS_WORDS.len())
            .map(|g| words.len() + guess_turns(words.iter().copied(), g))
            .filter(|t| *t != 0)
    ).unwrap_or((GUESS_WORDS.len(), 0))
}

#[derive(Clone, Debug)]
pub struct BestNode {
    pub(crate) best_guess: usize,
    pub(crate) turns: usize,
    pub(crate) complete: bool,

    pub(crate) branches: Vec<Result<AvgNode, usize>>
}

#[derive(Clone, Debug)]
pub struct AvgNode {
    pub(crate) turns: usize,

    pub(crate) hint_ordering: Vec<u8>,
    pub(crate) next_branch: usize,
    pub(crate) branches: Vec<Result<BestNode, usize>>
}

impl AvgNode {
    pub fn new(guess: usize, parent_words: &WordSet) -> Self {
        let mut out =
            Self {
                turns: 0,
                hint_ordering: Vec::new(),
                next_branch: 0,
                branches: Vec::new(),
            };

        out.turns = guess_turns(parent_words.iter(), guess);

        BUFFER.with(|buf| {
            let mut hints = buf
                .borrow()
                .iter()
                .enumerate()
                .map(|(x,y)| (-(*y as isize), x))
                .collect::<Vec<_>>();

            hints.sort_unstable();

            out.hint_ordering = hints
                .into_iter()
                .filter(|(x, _)| *x != 0)
                .map(|x| x.1 as u8)
                .collect();
        });

        out
    }

    pub fn update_turns(&mut self, guess: usize, parent_words: &WordSet) {
        let mut freqs = vec![0; HINT_POSSIBILITIES];

        get_hint_frequency(&mut freqs, parent_words.iter(), guess);

        self.turns = (0..self.hint_ordering.len())
            .map(|i| {
                let hint = self.hint_ordering[i] as usize;
                let n_words = freqs[hint];

                if n_words == 0 || hint == ALL_GREEN {
                    0
                } else if n_words == 1 {
                    1
                } else if i < self.branches.len() {
                    match self.branches[i] {
                        Ok(BestNode{turns: t, ..}) | Err(t) => t
                    }
                } else {
                    avg_turns(n_words)
                }
            })
            .sum();
    }
}

impl BestNode {
    // computes the number of turns remaining and the best word from "guess_turns"
    pub fn update_turns(&mut self, words: &WordSet) {
        let (loc, min) =
            loc_of_min(self.branches
                .iter()
                .map(|b| match b {
                    Ok(AvgNode{turns: t, ..}) | Err(t) => t
                }))
                .unwrap();

        self.best_guess = loc;
        self.turns = *min + words.len();
    }

    pub fn new(words: &WordSet) -> Self {
        let mut out =
            Self {
                best_guess: 0,
                turns: 0,
                complete: false,
                branches: vec![Err(0); GUESS_WORDS.len()],
            };

        for guess in 0..GUESS_WORDS.len() {
            out.branches[guess] = Err(guess_turns(words.iter(), guess));
        }

        out.update_turns(words);

        out
    }
}
