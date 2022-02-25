use crate::words::*;
use crate::word_sets::*;

use std::cell::RefCell;

pub fn avg_turns(words: usize) -> f64 {
    let mut words = words as f64;

    if words == 0. {
        return 0.;
    }

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

fn get_hint_frequency<I: Iterator<Item=usize>>(buf: &mut Vec<usize>, words: I, guess: usize) {
    for w in words {
        buf[TABLE[guess][w] as usize] += 1;
    }
}

fn weighted_average<I, J>(weights: I, nums: J) -> f64
    where I: Iterator<Item=f64>, J: Iterator<Item=f64>
{
    let mut out = 0.;
    let mut sum = 0.;

    for (weight, num) in weights.zip(nums) {
        if weight != 0. {
            out += weight * num;
            sum += weight;
        }
    }

    if sum == 0. {
        0.
    } else {
        out / sum
    }
}

thread_local!(static BUFFER: RefCell<Vec<usize>> = RefCell::new(vec![0; HINT_POSSIBILITIES]));

pub fn guess_turns<I: Iterator<Item=usize>>(words: I, guess: usize) -> f64 {
    let mut out = 0.;

    BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();

        buf.fill(0);

        get_hint_frequency(&mut buf, words, guess);

        let weights = buf.iter().map(|x| *x as f64);
        let turns = buf.iter().map(|w| avg_turns(*w));

        out = weighted_average(weights, turns);
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

pub fn best_turns(words: &BitSet) -> (usize, f64) {
    let words = words.iter().collect::<Vec<_>>();

    loc_of_min (
        (0..GUESS_WORDS.len())
            .map(|g| 1. + guess_turns(words.iter().copied(), g))
            .filter(|t| *t != 0.)
    ).unwrap_or((GUESS_WORDS.len(), 0.))
}

#[derive(Clone, Debug)]
pub struct BestNode {
    pub(crate) best_guess: usize,
    pub(crate) turns: f64,

    pub(crate) branches: Vec<Result<AvgNode, f64>>
}

#[derive(Clone, Debug)]
pub struct AvgNode {
    pub(crate) turns: f64,

    pub(crate) hint_ordering: Vec<u8>,
    pub(crate) next_branch: usize,
    pub(crate) branches: Vec<Result<BestNode, f64>>
}

impl AvgNode {
    pub fn new(guess: usize, parent_words: &BitSet) -> Self {
        let mut out =
            Self {
                turns: 0.,
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

    pub fn update_turns(&mut self, guess: usize, parent_words: &BitSet) {
        let mut freqs = vec![0; HINT_POSSIBILITIES];

        get_hint_frequency(&mut freqs, parent_words.iter(), guess);

        let weights = self.hint_ordering.iter().map(|x| freqs[*x as usize] as f64);
        let entropies = (0..self.hint_ordering.len())
            .map(|i| {
                let n_words = freqs[self.hint_ordering[i] as usize];

                if i < self.branches.len() {
                    match self.branches[i] {
                        Ok(BestNode{turns: t, ..}) | Err(t) => {
                            if n_words == 0 {
                                0.
                            } else if n_words == 1 {
                                1.
                            } else {
                                t
                            }
                        }
                    }
                } else {
                    avg_turns(n_words)
                }
            });

        self.turns = weighted_average(weights, entropies);
    }
}

impl BestNode {
    // computes the number of turns remaining and the best word from "guess_turns"
    pub fn update_turns(&mut self) {
        let (loc, min) =
            loc_of_min(self.branches
                .iter()
                .map(|b| match b {
                    Ok(AvgNode{turns: t, ..}) | Err(t) => t
                }))
                .unwrap();

        self.best_guess = loc;
        self.turns = *min + 1.;
    }

    pub fn new(words: &BitSet) -> Self {
        let mut out =
            Self {
                best_guess: 0,
                turns: 0.,
                branches: vec![Err(0.); GUESS_WORDS.len()],
            };

        for guess in 0..GUESS_WORDS.len() {
            out.branches[guess] = Err(guess_turns(words.iter(), guess));
        }

        out.update_turns();

        out
    }
}
