use crate::words::*;

use packed_simd::{u8x32, u64x4, IntoBits};

const BLOCK_BYTES: usize = 32;
const BLOCK_BITS : usize = BLOCK_BYTES * 8;

#[derive(Clone, Debug)]
pub struct BitSet {
    len: usize,
    pub(crate) bits: Vec<u8>
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            len: 0,
            bits: Vec::new()
        }
    }

    pub fn zeros(n: usize) -> Self {
        Self {
            len: n,
            bits: vec![0; (n / BLOCK_BITS + (n % BLOCK_BITS != 0) as usize) * BLOCK_BYTES],
        }
    }

    pub fn ones(n: usize) -> Self {
        let mut out = Self::new();

        out.len = n;
        out.bits = vec![0xff; n / BLOCK_BITS * BLOCK_BYTES];

        if n % 8 != 0 {
            out.bits.push((1 << n % 8) - 1)
        }

        while out.bits.len() % BLOCK_BYTES != 0 {
            out.bits.push(0);
        }

        out
    }

    // changes self into a copy of other, avoiding allocating if possible
    pub fn clone_from(&mut self, other: &Self) {
        self.len = other.len;
        self.bits.resize(other.bits.len(), 0);

        self.bits.clone_from_slice(&other.bits)
    }

    pub fn push(&mut self, item: bool) {
        if self.len % BLOCK_BITS == 0 {
            self.bits.push(0);

            while self.bits.len() % BLOCK_BYTES != 0 {
                self.bits.push(0);
            }
        }

        self.len += 1;

        if item {
            self.bits[self.len / 8] |= 1 << self.len % 8;
        }
    }

    pub fn add(&mut self, ind: usize) {
        self.bits[ind / 8] |= 1 << ind % 8
    }

    pub fn remove(&mut self, ind: usize) {
        self.bits[ind / 8] &= !(1 << ind % 8)
    }

    pub fn len(&self) -> usize {self.len}

    pub fn count_ones(&self) -> usize {
        let mut vec = u64x4::splat(0);

        for i in (0..self.bits.len()).step_by(32) {
            // cast to u64s to prevent overflows
            let vec2: u64x4 = u8x32::from_slice_unaligned(&self.bits[i..i+32]).into_bits();

            vec += vec2.count_ones();
        }

        vec.wrapping_sum() as usize
    }
}

use std::iter::{Extend, FromIterator};

impl Extend<bool> for BitSet {
    fn extend<T: IntoIterator<Item=bool>>(&mut self, iter: T) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl FromIterator<bool> for BitSet {
    fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> Self {
        let mut out = Self::new();

        out.extend(iter);

        out
    }
}

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        for i in (0..self.bits.len()).step_by(32) {
            let mut vec1 = u8x32::from_slice_unaligned(&self.bits[i..i+32]);
            let     vec2 = u8x32::from_slice_unaligned(&rhs .bits[i..i+32]);

            vec1 &= vec2;

            vec1.write_to_slice_unaligned(&mut self.bits[i..i+32])
        }
    }
}

impl BitAnd<&BitSet> for BitSet {
    type Output = Self;

    fn bitand(mut self, rhs: &Self) -> Self {
        self &= rhs;
        self
    }
}


impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in (0..self.bits.len()).step_by(32) {
            let mut vec1 = u8x32::from_slice_unaligned(&self.bits[i..i+32]);
            let     vec2 = u8x32::from_slice_unaligned(&rhs .bits[i..i+32]);

            vec1 |= vec2;

            vec1.write_to_slice_unaligned(&mut self.bits[i..i+32])
        }
    }
}

impl BitOr<&BitSet> for BitSet {
    type Output = Self;

    fn bitor(mut self, rhs: &Self) -> Self {
        self |= &rhs;
        self
    }
}
pub const HINT_POSSIBILITIES: usize = 243;

pub fn gen_guess_hint_table() -> Vec<Vec<BitSet>> {
    let mut out = Vec::with_capacity(GUESS_WORDS.len());

    for g in 0..GUESS_WORDS.len() {
        let mut hints: Vec<BitSet> = vec![BitSet::new(); HINT_POSSIBILITIES];

        for s in 0..SOLUTION_WORDS.len() {
            let ind = TABLE[g][s] as usize;

            if hints[ind].len() == 0 {
                let mut set = BitSet::zeros(SOLUTION_WORDS.len());

                set.add(s);

                hints[ind] = set;
            } else {
                hints[ind].add(s);
            }
        }

        out.push(hints);
    }

    out
}

lazy_static! {
    pub static ref GUESS_HINT_TABLE: Vec<Vec<BitSet>> = {
        gen_guess_hint_table()
    };
}

pub fn print_word_set(set: &BitSet) {
    for i in 0..set.len() {
        if set.bits[i / 8] & 1 << i % 8 != 0 {
            print!("{} ", SOLUTION_WORDS[i]);
        }
    }
    println!();
}

extern crate test;

#[test]
fn t_bitset() {
    assert_eq!(BitSet::ones(2309).bits.len(), 320)
}

#[bench]
fn b_bitset_union(b: &mut test::Bencher) {
    let mut set1 = BitSet::zeros(2309);
    let mut set2 = BitSet::zeros(2309);

    b.iter(|| {set1 &= &mut set2; test::black_box(set1.bits[0])})
}

#[bench]
fn b_bitset_count_ones(b: &mut test::Bencher) {
    let mut set1 = BitSet::ones(2309);

    b.iter(|| test::black_box(set1.count_ones()))
}
