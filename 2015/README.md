# Solution notes

## day 7

I originally solved this with [fn-cache](https://crates.io/crates/fn-cache) but that felt dissatisfying; I rewrote using [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html).

## day 15 part 2

This uses a hardcoded set of nested loops, so it doesn't generalize to recipes
with different numbers of ingredients.

## day 20 part 1

Takes a little over 2 mins on my machine, definitely not the intended
solution. By using the answer, a naive iterative solution completes
~instantly, but also obviously not intended.
