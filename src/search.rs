use crate::words::*;
use crate::word_sets::*;
use crate::searchtree::*;

use std::mem;

impl BestNode {
    pub fn search(&mut self) {
        let mut stack: Vec<*mut BestNode> = Vec::new();
        let mut ptr = self;

        // descend until we create a new tree
        while ptr.branches[ptr.best_guess].complete() {
            let guess = ptr.best_guess;
            let branch_idx = *ptr.branches[guess].next_branch();

            *ptr.branches[guess].next_branch_mut() += 1;
            *ptr.branches[guess].next_branch_mut() %= ptr.branches[guess].branches().len();

            stack.push(ptr as *mut BestNode);

            print!("{} > {} > ", GUESS_WORDS[guess], hint_to_str(ptr.branches[guess].hint_ordering()[branch_idx]));

            ptr = &mut ptr.branches[guess].branches_mut()[branch_idx];
        }

        // create the new tree
        let guess = ptr.best_guess;

        println!("{}", GUESS_WORDS[guess]);

        ptr.branches[guess].init_hint_ordering(guess, &ptr.words);

        let hint = ptr.branches[guess].hint_ordering()[ptr.branches[guess].branches().len()] as usize;
        let tree = BestNode::new(ptr.words.clone() & &GUESS_HINT_TABLE[guess][hint]);

        stack.push(ptr as *mut BestNode);

        ptr.branches[guess].branches_mut().push(tree);

        mem::drop(ptr);

        // ascend and update all parent trees
        while let Some(ptr) = stack.pop() {
            let ptr = unsafe{ptr.as_mut()}.unwrap();

            let guess = ptr.best_guess;

            ptr.branches[guess].update_turns(guess, &ptr.words);
            ptr.update_turns();
        }
    }
}
