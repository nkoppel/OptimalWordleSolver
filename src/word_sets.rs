use crate::words::*;

use std::{iter::Cloned, slice::Iter};

#[derive(Clone, Debug)]
pub struct WordSet(pub Vec<usize>);

impl WordSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn all(n_words: usize) -> Self {
        Self((0..n_words).collect::<Vec<_>>())
    }

    pub fn iter(&self) -> Cloned<Iter<usize>> {
        self.0.iter().cloned()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, word: usize) {
        if let Err(i) = self.0.binary_search(&word) {
            self.0.insert(i, word)
        }
    }

    pub fn remove(&mut self, word: usize) {
        if let Ok(i) = self.0.binary_search(&word) {
            self.0.remove(i);
        }
    }

    pub fn reduce_mut(&mut self, guess: usize, hint: u8) {
        let guess_table = &TABLE[guess];

        self.0.retain(|w| guess_table[*w] == hint);
    }

    pub fn reduce(mut self, guess: usize, hint: u8) -> Self {
        self.reduce_mut(guess, hint);

        self
    }
}

use std::fmt;

impl fmt::Display for WordSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for w in &self.0 {
            write!(f, "{} ", SOLUTION_WORDS[*w])?;
        }

        Ok(())
    }
}
