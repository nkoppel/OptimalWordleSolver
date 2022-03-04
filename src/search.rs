use crate::words::*;
use crate::word_sets::*;
use crate::searchtree::*;

use std::io::Write;

fn is_complete(r: &Result<AvgNode, f64>) -> bool {
    if let Ok(node) = r {
        node.branches.len() == node.hint_ordering.len()
    } else {
        false
    }
}

impl BestNode {
    pub fn search(&mut self, words: WordSet) -> bool {
        if is_complete(&self.branches[self.best_guess]) {
            // descend
            let guess = self.best_guess;
            let avg = self.branches[guess].as_mut().unwrap();
            let orig_branch = avg.next_branch;

            let mut exit = false;

            loop {
                let branch_idx = avg.next_branch;

                if exit && branch_idx == orig_branch {
                    self.complete = true;
                    return false;
                }
                exit = true;

                avg.next_branch += 1;
                avg.next_branch %= avg.branches.len();

                print!("{} > {} > ", GUESS_WORDS[guess], hint_to_str(avg.hint_ordering[branch_idx]));

                if let Ok(BestNode{complete: true, ..}) = avg.branches[branch_idx] {
                    println!("EXIT");
                    continue
                }

                let new_words = words.clone().reduce(guess, avg.hint_ordering[branch_idx]);

                if new_words.len() <= 1 {
                    println!("EXIT");
                    continue
                }

                if avg.branches[branch_idx].is_err() {
                    avg.branches[branch_idx] = Ok(BestNode::new(&new_words));
                }

                if avg.branches[branch_idx].as_mut().unwrap().search(new_words) {
                    break;
                }
            }

            // propogate new turn values
            avg.update_turns(guess, &words);
            self.update_turns();
            true
        } else {
            // create a new tree
            let guess = self.best_guess;

            println!("{}", GUESS_WORDS[guess]);

            if self.branches[guess].is_err() {
                self.branches[guess] = Ok(AvgNode::new(guess, &words));
            }

            let avg = self.branches[guess].as_mut().unwrap();

            if avg.hint_ordering.is_empty() {
                return false;
            }

            let new_words = words.clone().reduce(guess, avg.hint_ordering[avg.branches.len()]);

            avg.branches.push(Err(best_turns(&new_words).1));

            // propogate new turn values
            avg.update_turns(guess, &words);
            self.update_turns();

            true
        }
    }

    pub fn write_strategy<W: Write>(&self, words: WordSet, prefix: &str, w: &mut W) {
        let guess = self.best_guess;
        let avg = self.branches[guess].as_ref().unwrap();

        for i in 0..avg.branches.len() {
            let hint = avg.hint_ordering[i];
            let prefix = format!("{}{} {} ", prefix, GUESS_WORDS[guess], hint_to_str(hint));

            let new_words = words.clone().reduce(guess, hint);

            if let Ok(branch) = &avg.branches[i] {
                branch.write_strategy(new_words, &prefix, w);
            } else if hint as usize != ALL_GREEN {
                writeln!(w, "{}{} ggggg", prefix, SOLUTION_WORDS[new_words.iter().next().unwrap()]).unwrap();
            } else {
                writeln!(w, "{}", prefix.trim()).unwrap();
            }
        }
    }
}
