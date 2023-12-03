# Solution notes

## Day 1

I got trapped by both parts. In the first part, I ended up with an awful regex like `/\D*(\d).*?(\d?)\D*$/`. In the second part, that regex led me astray; I preprocessed each line by replacing each word from left to right. The problem was sneakily-worded to make that seem viable.

## Day 2

Stumbled on min vs max for part 2, but otherwise went smoothly!

## Day 3

Part 1 was a nightmare. I kept getting the example correct, but failing on the real input. Tried a lot of things: using negative numbers, treating digits as symbols, etc. I even rewrote my solution into a different language to try to figure it out. Turns out I had two different off-by-one errors (one carried over into my Perl). Part 2 was quick and painless.

# Results

| Day | #1 Time  | #1 Rank | #2 Time  | #2 Rank |
| --- | -------- | ------- | -------- | ------- |
| 1   | 00:09:49 | 4833    | 00:41:45 | 4721    |
| 2   | 00:04:49 | 156     | 00:09:14 | 392     |
| 3   | 01:15:13 | 8424    | 01:20:21 | 5998    |
