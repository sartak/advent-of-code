# Solution notes

## Day 1

I got trapped by both parts. In the first part, I ended up with an awful regex like `/\D*(\d).*?(\d?)\D*$/`. In the second part, that regex led me astray; I preprocessed each line by replacing each word from left to right. The problem was sneakily-worded to make that seem viable.

## Day 2

Stumbled on min vs max for part 2, but otherwise went smoothly!

## Day 3

Part 1 was a nightmare. I kept getting the example correct, but failing on the real input. Tried a lot of things: using negative numbers, treating digits as symbols, etc. I even rewrote my solution into a different language to try to figure it out. Turns out I had two different off-by-one errors (one carried over into my Perl). Part 2 was quick and painless.

# Day 4

Part 1 was quick and easy. Part 2 I ended up with an exponential solution (maintain a queue of card IDs to process; I could have at least cached the score of each card…). I had a working solution at 00:07:31 (which would have put me somewhere between rank 101-200), but it took too long to run so I assumed I hit an infinite loop. I faffed around for a bit, ran a release build (which still took 5 seconds), and got the star at 14 mins. (I'm much happier with the much better time complexity of my tidied solution.)

# Day 5

Part 1 was fine. A little overcomplicated perhaps. If needed I could have trivally extended my solution to handle multiple branching paths. Unfortunately that was not the direction part 2 went.

Part 2 was brutal. Learning from my lesson yesterday, I left the naive solution running in the background for a while, but they wouldn't let me get away with that this time. It took me about three minutes to restructure the code to support ranges, but then I spent nearly all the two hours writing and debugging the range intersection code. I was able to simplify the problem by handling only one segment of the range (the beginning) at a time, but that still left [8 branches to debug](https://github.com/sartak/advent-of-code/blob/a4b897d15158602ed9c8045f603752b4d44d4eb7/2023/src/bin/05b.rs#L74-L111). So many off-by-one errors. I eventually cracked it by using a very small input range (`81..2`) and carefully verifying how each step transformed it. The problem branch was the last one (which assigns `nl`).

Unlike day 3, I never really felt frustrated. The problem was wrapping my head around the range logic, rather than having some unknown bug somewhere with no great debugging tools. So, even though it took me far longer to solve, I greatly preferred today over day 3.

# Results

| Day | #1 Time  | #1 Rank | #2 Time  | #2 Rank |
| --- | -------- | ------- | -------- | ------- |
| 1   | 00:09:49 | 4833    | 00:41:45 | 4721    |
| 2   | 00:04:49 | 156     | 00:09:14 | 392     |
| 3   | 01:15:13 | 8424    | 01:20:21 | 5998    |
| 4   | 00:03:43 | 268     | 00:14:04 | 979     |
| 5   | 00:22:20 | 1703    | 02:21:06 | 5161    |
