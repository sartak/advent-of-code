# Solution notes

## Day 1

I got trapped by both parts. In the first part, I ended up with an awful regex like `/\D*(\d).*?(\d?)\D*$/`. In the second part, that regex led me astray; I preprocessed each line by replacing each word from left to right. The problem was sneakily-worded to make that seem viable.

# Results

| Day | Part 1 Time | Part 2 Time |
| --- | ----------- | ----------- |
| 1   | 00:09:49    | 00:41:45    |
