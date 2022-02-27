# Perfect Wordle Solver

Uses an optimized brute-force search to find strategies for [wordle](https://www.nytimes.com/games/wordle/index.html) with the minimum possible average game length. From it's computation, the optimal starting word is "salet", and the optimal average game length is 3.4223.

There are two included solution files, "2309\_solution.txt", which contains the strategy for the case where only solution words are guessable, and "full\_solution.txt", which contains the strategy using all guess words. On a single core, the former takes three minutes and 2.5 gigabytes of memory, while the latter takes and hour and 10 minutes and about 20 gigabytes of memory. I have not yet implemented any interface for interacting with these solution files, so for now searching through them with control-f works.

## How it works

The core idea behind this program is that we don't have to spend any time on a guess word if we know it to be worse than the best guess word. To that end, the program begins by computing a lower bound of the game length after each guess, by finding a weighted average of the minimum game length after each hint. It then repeatedly increases the lower bound of the guess with the lowest lower bound by applying the same process to one of the resulting lists of words after the best guess and a hint, doing this recursively if necessary. In this way, it spares a huge amount of computation by only spending time searching the word it thinks to be the best each iteration.
