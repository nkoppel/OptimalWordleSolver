use std::collections::HashMap;

pub fn read_words(filename: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(filename).unwrap();
    let mut out = Vec::new();

    for line in BufReader::new(file).lines() {
        out.push(line.unwrap().trim().to_string());
    }

    out
}

// Returns the hint that would be returned by wordle given a guess and the secret word.
// Output is a ternary number with twos representing green letters and ones repersenting
// yellow letters
fn get_hint<T: AsRef<[u8]>>(guess: T, answer: T) -> u8 {
    let guess : &[u8] = guess .as_ref();
    let answer: &[u8] = answer.as_ref();
    let mut out = 0;

    for (i, c) in guess.iter().enumerate() {
        out *= 3;

        if *c == answer[i] {
            out += 2;
        } else if answer.contains(c) {
            let g_count =
                guess[0..i].iter().filter(|x| *x == c).count() +
                (i..guess.len()).filter(|i| guess[*i] == answer[*i] && guess[*i] == *c).count();

            let a_count = answer.iter().filter(|x| *x == c).count();

            if g_count < a_count {
                out += 1;
            }
        }
    }

    out
}

pub fn get_word_map(words: Vec<String>) -> HashMap<String, usize> {
    words.into_iter().enumerate().map(|(v, k)| (k, v)).collect()
}

pub fn gen_hint_table<T: AsRef<[u8]>>(solutions: &[T], guesses: &[T]) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; solutions.len()]; guesses.len()];

    for (out_row, guess) in out.iter_mut().zip(guesses.iter()) {
        for (o, solution) in out_row.iter_mut().zip(solutions.iter()) {
            *o = get_hint(guess, solution);
        }
    }

    out
}

pub fn hint_from_str(s: &str) -> u8 {
    let mut out = 0;

    for c in s.chars() {
        out *= 3;

        match c {
            'g' => out += 2,
            'y' => out += 1,
            _ => {}
        }
    }

    out
}

pub fn hint_to_str(mut hint: u8) -> String {
    let mut out = String::new();

    for _ in 0..5 {
        match hint % 3 {
            2 => out.push('g'),
            1 => out.push('y'),
            _ => out.push('_'),
        }

        hint /= 3;
    }

    out.chars().rev().collect::<String>()
}

lazy_static! {
    // pub static ref GUESS_WORDS   : Vec<String> = read_words("words/guess_words.txt"   );
    // pub static ref GUESS_WORDS   : Vec<String> = read_words("words/solution_words.txt");
    // pub static ref SOLUTION_WORDS: Vec<String> = read_words("words/solution_words.txt");

    pub static ref GUESS_WORDS   : Vec<String> = read_words("words/guess_words_2.txt"   );
    pub static ref SOLUTION_WORDS: Vec<String> = read_words("words/solution_words_2.txt");

    // pub static ref GUESS_WORDS   : Vec<String> = read_words("words/testing_words.txt");
    // pub static ref SOLUTION_WORDS: Vec<String> = read_words("words/testing_words.txt");

    pub static ref GUESS_MAP:    HashMap<String, usize> = get_word_map(GUESS_WORDS   .clone());
    pub static ref SOLUTION_MAP: HashMap<String, usize> = get_word_map(SOLUTION_WORDS.clone());

    // gives the hint corresponding to a word and a guess when indexed by the guess, then by the
    // word
    pub static ref TABLE: Vec<Vec<u8>> = gen_hint_table(&SOLUTION_WORDS, &GUESS_WORDS);
}

extern crate test;

#[test]
fn t_get_hint() {
    assert_eq!(get_hint("eagle", "speed"), hint_from_str("y___y"));
    assert_eq!(get_hint("bbaba", "bcbcc"), hint_from_str("gy___"));
    assert_eq!(get_hint("eagle", "reads"), hint_from_str("yy___"));
    assert_eq!(get_hint("perch", "bench"), hint_from_str("_g_gg"));
    assert_eq!(get_hint("treat", "moist"), hint_from_str("____g"));

    assert_eq!("_g_gg".to_string(), hint_to_str(hint_from_str("bgbgg")));
    assert_eq!(128, hint_from_str(&hint_to_str(128)));
}

#[bench]
fn b_get_hint(b: &mut test::Bencher) {
    use test::black_box;
    b.iter(|| get_hint(black_box("eagle"), black_box("reads")))
}

// #[bench]
// fn b_gen_hint_table(b: &mut test::Bencher) {
    // use test::black_box;
    // b.iter(|| gen_hint_table(black_box(&SOLUTION_WORDS), black_box(&GUESS_WORDS)))
// }
