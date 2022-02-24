use crate::words::*;
use crate::word_sets::*;
use crate::searchtree::*;

use std::mem;

fn is_complete(r: &Result<AvgNode, f64>) -> bool {
    if let Ok(node) = r {
        node.branches.len() == node.hint_ordering.len()
    } else {
        false
    }
}

impl BestNode {
    pub fn search(&mut self) {
        let mut stack: Vec<(*mut BestNode, BitSet)> = Vec::new();
        let mut words = BitSet::ones(SOLUTION_WORDS.len());
        let mut ptr = self;

        // descend until we can create a new tree
        while is_complete(&ptr.branches[ptr.best_guess]) {
            stack.push((ptr as *mut BestNode, words.clone()));

            let guess = ptr.best_guess;
            let avg = ptr.branches[guess].as_mut().unwrap();
            let branch_idx = avg.next_branch;

            avg.next_branch += 1;
            avg.next_branch %= avg.branches.len();

            print!("{} > {} > ", GUESS_WORDS[guess], hint_to_str(avg.hint_ordering[branch_idx]));

            words &= &GUESS_HINT_TABLE[guess][avg.hint_ordering[branch_idx] as usize];

            if words.count_ones() <= 1 {
                println!("EXIT");
                return;
            }

            if avg.branches[branch_idx].is_err() {
                avg.branches[branch_idx] = Ok(BestNode::new(&words));
            }

            ptr = avg.branches[branch_idx].as_mut().unwrap();
        }

        stack.push((ptr as *mut BestNode, words.clone()));

        // create a new tree
        let guess = ptr.best_guess;

        println!("{}", GUESS_WORDS[guess]);

        if ptr.branches[guess].is_err() {
            ptr.branches[guess] = Ok(AvgNode::new(guess, &words));
        }

        let avg = ptr.branches[guess].as_mut().unwrap();

        if avg.hint_ordering.len() == 0 {
            return;
        }

        // println!("{:?}", avg);
        words &= &GUESS_HINT_TABLE[guess][avg.hint_ordering[avg.branches.len()] as usize];

        avg.branches.push(Err(best_entropy(&words).1));

        mem::drop(ptr);

        // propogate error upwards
        for (ptr, words) in stack.into_iter().rev() {
            let ptr = unsafe{ptr.as_mut()}.unwrap();

            let guess = ptr.best_guess;
            let avg = ptr.branches[guess].as_mut().unwrap();

            avg.update_entropy(guess, &words);
            ptr.update_entropy();
        }
    }
}
